#![forbid(unsafe_code)]

use chrono::prelude::*;
use std::net::IpAddr;

use crate::ErrorType;

use super::request_manager::Request;

/// Enum for handling log statement for printer
pub enum LogStatement {
    Request(Request),
    NewConnection,
    Shutdown,
    Error(ErrorType),
}

/// Struct for easier log print
pub struct Logger {
    pub ip: IpAddr,
    pub state: LogStatement,
    pub cur_storage_size: usize,
}

/// Function that prints given '''Logger''' statement message with additional information.
pub fn print_log(logger: Logger) {
    print!("{} [{}] ", logger.ip, Utc::now().format("%d/%b%Y:%T %z"),);

    match Some(logger.state) {
        Some(LogStatement::Request(request)) => match request {
            Request::Store { key, hash } => {
                print!(
                    "Received request to write new value {} by key {}. ",
                    hash, key
                );
            }
            Request::Load { key } => {
                print!("Received request to get value by key {}. ", key);
            }
        },
        Some(LogStatement::NewConnection) => {
            print!("Connection established. ");
        }
        Some(LogStatement::Shutdown) => unimplemented!(),
        Some(LogStatement::Error(err)) => match err {
            ErrorType::BadReading => {
                println!("Can't read request from client ");
            }
            ErrorType::IoError(_) => {
                println!("Can't read(write) request(response) to client ");
            }
            ErrorType::JsonErr(_) => {
                println!("Client sent bad json request ");
            }
            ErrorType::BadConnection => {
                println!("Can't run server with given ip and port ");
            }
        },
        None => unimplemented!(),
    }

    println!("Storage size: {}.", logger.cur_storage_size);
}
