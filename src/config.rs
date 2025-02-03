use std::{path, path::PathBuf};

const DEFAULT_PORT: u16 = 7878;

#[derive(clap::Parser, Debug)]
pub struct Config {
    #[arg(short, long, env, default_value_t = DEFAULT_PORT)]
    pub port: u16,

    #[arg(value_parser = directory_parser, default_value = ".")]
    pub serve_dir: PathBuf,
}

pub fn directory_parser(input: &str) -> Result<PathBuf, &'static str> {
    let path = path::Path::new(input);

    if !path.is_dir() {
        return Err(
            "Invalid path: Either does not exist or is not a directory",
        );
    }

    path::Path::canonicalize(path)
        .map_err(|_| "Failed to resolve absolute path")
}
