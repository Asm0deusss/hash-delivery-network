#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr, TcpListener},
    sync::{Arc, Mutex},
    thread,
};

use super::{
    json_manager::{get_request, send_response, Request, Response},
    log_printer::{print_log, LogStatement},
};

pub fn run(ip: IpAddr, port: u16) {
    let map: HashMap<String, String> = HashMap::new();
    let map_ref = Arc::new(Mutex::new(map));

    let address = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        if stream.is_err() {
            continue;
        }

        let mut stream = stream.unwrap();
        let hash_to_key = Arc::clone(&map_ref);

        print_log(
            ip,
            LogStatement::NewConnection,
            hash_to_key.lock().unwrap().len(),
        );

        let _ = thread::spawn(move || loop {
            let request = get_request(&mut stream);

            let request = match request {
                Ok(req) => req,
                Err(_) => {
                    send_response(&mut stream, Response::Err);
                    break;
                }
            };

            print_log(
                ip,
                LogStatement::Request(&request),
                hash_to_key.lock().unwrap().len(),
            );

            let response = match request {
                Request::Store { key, hash } => {
                    let mut guard = hash_to_key.lock().unwrap();
                    guard.insert(key.clone(), hash.clone());
                    Response::SuccessLoad {
                        key: (key),
                        hash: (hash),
                    }
                }
                Request::Load { key } => {
                    let guard = hash_to_key.lock().unwrap();
                    match guard.get(&key) {
                        Some(hash) => Response::SuccessLoad {
                            key: (key),
                            hash: (hash.to_owned()),
                        },
                        None => Response::NoKey,
                    }
                }
            };

            send_response(&mut stream, response);
        });
    }
}
