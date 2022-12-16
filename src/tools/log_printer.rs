#![forbid(unsafe_code)]

use chrono::prelude::*;
use std::net::IpAddr;

use super::request_manager::Request;

/// Enum for easier log output
pub enum LogStatement<'a> {
    Request(&'a Request),
    NewConnection,
}

/// Function that prints given '''LogStatement''' message with additional information.
pub fn print_log(ip: IpAddr, state: LogStatement, cur_storage_size: usize) {
    print!("{} [{}] ", ip, Utc::now().format("%d/%b%Y:%T %z"),);

    match state {
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
    }

    println!("Storage size: {}.", cur_storage_size);
}
