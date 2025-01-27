const DEFAULT_PORT: u16 = 7878;

#[derive(clap::Parser, Debug)]
pub struct Config {
    #[arg(short, long, env, default_value_t = DEFAULT_PORT)]
    pub port: u16
}
