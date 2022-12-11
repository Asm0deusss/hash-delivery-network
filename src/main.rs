#![forbid(unsafe_code)]

use clap::Parser;
use hash_delivery_service::tools::run::run;

use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Parser)]
struct Opts {
    #[clap(short, long)]
    ip: IpAddr,

    #[clap(short, long, default_value = "0")]
    port: u16,
}

fn main() {
    let ip4 = Ipv4Addr::new(127, 0, 0, 1);
    let ip = IpAddr::V4((ip4));

    run(ip, 8888);
}
