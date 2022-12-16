#![forbid(unsafe_code)]

use clap::Parser;
use hash_delivery_service::run::run;

use std::net::IpAddr;

/// Structure for running server from terminal
#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long, default_value = "127.0.0.1")]
    ip: IpAddr,

    #[clap(short, long, default_value = "8888")]
    port: u16,
}

fn main() {
    let opts = Opts::parse();
    let run = run(opts.ip, opts.port);

    if run.is_err() {
        println!(
            "Can't start the server with ip {} and port {}",
            opts.ip, opts.port
        );
    }
}
