//! Server to control and manage Simon's Column lights.

mod choreography;
mod config;
mod instrumentation;
mod orchestrator;
mod web;

use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();
    instrumentation::setup(&config);

    web::serve(&config).await.expect("serve web requests");
}
