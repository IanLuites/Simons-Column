//! Server to control and manage Simon's Column lights.

mod config;
use config::Config;

mod web;

#[tokio::main]
async fn main() {
    let config = Config::load();

    println!("Hello, world!");
    web::serve(&config).await.expect("serve web requests");
}
