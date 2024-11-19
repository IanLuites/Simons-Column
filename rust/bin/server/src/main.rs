//! Server to control and manage Simon's Column lights.

mod config;
use config::Config;

mod instrumentation;

mod web;

#[tokio::main]
async fn main() {
    let config = Config::load();
    instrumentation::setup(&config);

    web::serve(&config).await.expect("serve web requests");
}
