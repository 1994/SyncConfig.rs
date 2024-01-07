use std::thread::{self};

use clap::{arg, Parser};
use simple_logger::SimpleLogger;

mod sync;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // location of config file
    #[arg(short, long)]
    config: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()?;
    let config = sync::Config::new(&args.config)?;
    loop {
        config.sync()?;
        thread::sleep(config.get_update_duration())
    }
}
