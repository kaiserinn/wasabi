use clap::Parser;
use wasabi::config::Config;

fn main() {
    let config = Config::parse();

    println!("Listening on port {}", config.port);
    wasabi::listen(("localhost", config.port));
}
