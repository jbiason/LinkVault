//! Command line arguments

use std::net::IpAddr;

#[derive(clap::Parser, Debug)]
pub struct Cli {
    /// Binding address
    #[arg(long, short, default_value = "127.0.0.1")]
    pub address: IpAddr,

    /// Binding port
    #[arg(long, short, default_value = "3000")]
    pub port: u16,
}
