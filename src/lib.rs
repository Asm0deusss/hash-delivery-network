#![forbid(unsafe_code)]
//! Implementation of a simple Hash Delivery Network.
//! [Read more](https://imported-sofa-e34.notion.site/2-Hash-delivery-network-05023f0157af495ab12df85bca0b8d79)
//! # Protocol of communication
//! All communication between the client and the server takes place through messages in json format.
//! ## Store
//! Clients request:
//! ```
//! {
//!   "request_type": "store",
//!   "key": "some_key",
//!   "hash": "some_hash"
//! }
//! ```
//! Server response:
//! ```
//! {
//!   "response_status": "success"
//! }
//! ```
//!
//! ## Load
//! Clients request:
//! ```
//! {
//!   "request_type": "load",
//!   "key": "some_key"
//! }
//! ```
//! Server response if there are ```hash``` under requested ```key```:
//! ```
//! {
//!   "response_status": "success",
//!   "requested_key": "some_key",
//!   "requested_hash": "0b672dd94fd3da6a8d404b66ee3f0c83",
//! }
//! ```
//! Otherwise
//! ```
//! {
//!   "response_status": "key not found",
//! }
//! ```

pub mod run;
pub mod tools;

use core::fmt;

/// Enum for error handling.
pub enum ErrorType {
    /// This error will accure if server can't read any bytes from the client.
    BadReading,

    /// Type for ```std::io::Error``` errors.
    IoError(std::io::Error),

    /// Type for ```serde_json::Error``` errors.
    JsonErr(serde_json::Error),

    /// This error will accure if server can't start with given ```ip``` and ```port```
    BadConnection,
}

/// Implementation of ```fmt::Display```
impl fmt::Display for ErrorType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

/// Implementation of ```std::io::Error``` errors.
impl From<std::io::Error> for ErrorType {
    fn from(err: std::io::Error) -> Self {
        ErrorType::IoError(err)
    }
}

/// Implementation of ```serde_json::Error``` errors.
impl From<serde_json::Error> for ErrorType {
    fn from(err: serde_json::Error) -> Self {
        ErrorType::JsonErr(err)
    }
}
