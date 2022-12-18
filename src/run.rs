#![forbid(unsafe_code)]
//! Module with function [`run`](crate::run::run), which will launch the server
use std::{
    collections::HashMap,
    io::Write,
    net::{IpAddr, SocketAddr, TcpListener},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use serde_json::json;

use crate::{
    tools::log_printer::{print_log, Logger},
    ErrorType,
};

use super::{
    tools::log_printer::LogStatement,
    tools::request_manager::{get_request, send_response, Request, Response},
};

/// Function that wll lauch server with given ```ip``` and ```port```
pub fn run(ip: IpAddr, port: u16) -> Result<(), ErrorType> {
    let map: HashMap<String, String> = HashMap::new();
    let map = Arc::new(Mutex::new(map));

    let (sender, reciever): (Sender<Logger>, Receiver<Logger>) = channel();

    let address = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(address)?;

    let log_printer_thread = thread::spawn(move || loop {
        let log_statement = reciever.recv().unwrap();
        if log_statement.state == LogStatement::Shutdown {
            break;
        }
        print_log(log_statement);
    });

    for stream in listener.incoming() {
        if stream.is_err() {
            continue;
        }

        let mut stream = stream.unwrap();
        let map = Arc::clone(&map);

        let greeting = json!({"student_name" : "Gordei Skorobogatov"});
        stream.write_all(greeting.to_string().as_bytes())?;

        sender
            .clone()
            .send(Logger {
                ip,
                state: LogStatement::NewConnection,
                cur_storage_size: map.lock().unwrap().len(),
            })
            .unwrap();

        let sender = sender.clone();

        let current_thread = thread::spawn(move || loop {
            let request = get_request(&mut stream);

            let request = match request {
                Ok(req) => req,
                Err(_) => {
                    let response: Response = Response::Err;
                    let _ = send_response(&mut stream, response);
                    break;
                }
            };

            sender
                .send(Logger {
                    ip,
                    state: LogStatement::Request(request.clone()),
                    cur_storage_size: map.lock().unwrap().len(),
                })
                .unwrap();

            let response = match request {
                Request::Store { key, hash } => {
                    let mut guard = map.lock().unwrap();
                    guard.insert(key.clone(), hash.clone());
                    Response::SuccessStore {}
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

    sender
        .send(Logger {
            ip,
            state: LogStatement::Shutdown,
            cur_storage_size: 0,
        })
        .unwrap();
    log_printer_thread.join().unwrap();

    Ok(())
}
