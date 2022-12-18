#![forbid(unsafe_code)]

use chrono::prelude::*;
use std::net::IpAddr;

use super::request_manager::Request;

/// Enum for handling log statement for printer
#[derive(PartialEq)]
pub enum LogStatement {
    Request(Request),
    NewConnection,
    Shutdown,
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

    match logger.state {
        LogStatement::Request(request) => match request {
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
        LogStatement::NewConnection => {
            print!("Connection established. ");
        }
        LogStatement::Shutdown => unimplemented!(),
    }

    println!("Storage size: {}.", logger.cur_storage_size);
}
