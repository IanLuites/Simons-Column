//! Server to control and manage Simon's Column lights.

mod config;
use config::Config;

fn main() {
    let _config = Config::load();

    println!("Hello, world!");
}
