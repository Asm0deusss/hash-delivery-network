#![forbid(unsafe_code)]
//! Module with function [`run`](crate::run::run), which will launch the server
use std::{
    collections::HashMap,
    io::Write,
    net::{IpAddr, SocketAddr, TcpListener},
    sync::{Arc, Mutex},
    thread,
};

use serde_json::json;

use crate::ErrorType;

use super::{
    tools::log_printer::{print_log, LogStatement},
    tools::request_manager::{get_request, send_response, Request, Response},
};

/// Function that wll lauch server with given ```ip``` and ```port```
pub fn run(ip: IpAddr, port: u16) -> Result<(), ErrorType> {
    let map: HashMap<String, String> = HashMap::new();
    let map = Arc::new(Mutex::new(map));

    let address = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(address)?;

    for stream in listener.incoming() {
        if stream.is_err() {
            continue;
        }

        let mut stream = stream.unwrap();
        let map = Arc::clone(&map);

        let greeting = json!({"student_name" : "Gordei Skorobogatov"});
        stream.write_all(greeting.to_string().as_bytes())?;

        print_log(ip, LogStatement::NewConnection, map.lock().unwrap().len());

        let current_thread = thread::spawn(move || loop {
            let request = get_request(&mut stream);

            let request = match request {
                Ok(req) => req,
                Err(_) => {
                    break;
                }
            };

            print_log(
                ip,
                LogStatement::Request(&request),
                map.lock().unwrap().len(),
            );

            let response = match request {
                Request::Store { key, hash } => {
                    let mut guard = map.lock().unwrap();
                    guard.insert(key.clone(), hash.clone());
                    Response::SuccessLoad {
                        key: (key),
                        hash: (hash),
                    }
                }
                Request::Load { key } => {
                    let guard = map.lock().unwrap();
                    match guard.get(&key) {
                        Some(hash) => Response::SuccessLoad {
                            key: (key),
                            hash: (hash.to_owned()),
                        },
                        None => Response::NoKey,
                    }
                }
            };

            let send = send_response(&mut stream, response);

            if send.is_err() {
                break;
            }
        });
        current_thread.join().unwrap();
    }

    Ok(())
}
