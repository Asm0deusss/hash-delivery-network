#![forbid(unsafe_code)]
use crate::ErrorType;
use serde::{Deserialize, Serialize};

/// Enum for getting requests from client.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "request_type")]
pub enum Request {
    /// Store request.
    /// Gets pair (```key```, ```value```) to store.
    Store { key: String, hash: String },
    /// Load request.
    /// Gets ```key``` to load from server's storage.
    Load { key: String },
}

/// Enum for sending responses back to client.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "response_status")]
pub enum Response {
    /// Server will send it after a successful store.
    #[serde(rename = "success")]
    SuccessStore,
    /// Server will send it with requested ```key``` after a successful load.
    #[serde(rename = "success")]
    SuccessLoad {
        #[serde(rename = "requested_key")]
        key: String,
        #[serde(rename = "requested_hash")]
        hash: String,
    },
    #[serde(rename = "key not found")]
    /// Server will send it after a successful load.
    /// This response means that ```key``` are not in server's storage.
    NoKey,

    /// Server will send it after any problem.
    Err,
}

use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

/// Function for handling requests from client and ```serde``` them into ```Request``` type
pub fn get_request(stream: &mut TcpStream) -> Result<Request, ErrorType> {
    let mut read = BufReader::new(stream);
    let mut data: Vec<u8> = vec![];
    let bytes_read = read.read_until(b'}', &mut data)?;

    if bytes_read == 0 {
        return Err(ErrorType::BadReading);
    }

    let request: Request = serde_json::from_slice(&data)?;

    Ok(request)
}

/// Function for sending response to client
pub fn send_response(stream: &mut TcpStream, response: Response) -> Result<(), ErrorType> {
    stream.write_all(serde_json::to_string(&response)?.as_bytes())?;
    Ok(())
}
