#![forbid(unsafe_code)]

use crate::ErrorType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "request_type")]
pub enum Request {
    Store { key: String, hash: String },
    Load { key: String },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "response_status")]
pub enum Response {
    #[serde(rename = "success")]
    SuccessStore,
    #[serde(rename = "success")]
    SuccessLoad {
        #[serde(rename = "requested_key")]
        key: String,
        #[serde(rename = "requested_hash")]
        hash: String,
    },
    #[serde(rename = "key not found")]
    NoKey,
    Err,
}

use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub fn get_request(stream: &mut TcpStream) -> Result<Request, ErrorType> {
    let mut read = BufReader::new(stream);
    let mut data: Vec<u8> = vec![];
    let readed_bytes = read.read_until(b'}', &mut data)?;

    if readed_bytes == 0 {
        return Err(ErrorType::BadReading);
    }

    let request: Request = serde_json::from_slice(&data)?;

    Ok(request)
}

pub fn send_response(stream: &mut TcpStream, response: Response) -> Result<(), ErrorType> {
    stream.write_all(serde_json::to_string(&response)?.as_bytes())?;
    Ok(())
}
