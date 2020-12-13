#![feature(generators, generator_trait)]
use clap::Clap;
use std::fs::read_to_string;
use crate::server::Server;
use serde::Deserialize;
pub mod game;
pub mod server;

#[derive(Deserialize)]
pub struct Config {
    wroot: String,
}
#[tokio::main]
async fn main() {
    env_logger::init();
    let opts: Opts = Opts::parse();
    let config:Config = toml::from_str(read_to_string(opts.config).unwrap().as_str()).unwrap();
    let server = Server::new(opts.port);
    println!("Running housie on {}",opts.port);
    server.run(config).await;
}
#[derive(Clap,Debug)]
#[clap(version = "0.1.0", author = "Atmaram Naik <atmnk@yahoo.com>")]
struct Opts {
    #[clap(short, long, default_value = "8888")]
    port:u16,

    #[clap(short, long, default_value = "/usr/local/etc/tambola.toml")]
    config:String
}