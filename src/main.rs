use clap::Parser;
use wasabi::config::Config;

fn main() {
    let config = Config::parse();

    wasabi::serve(config);
}
