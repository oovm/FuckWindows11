use std::path::PathBuf;

use clap::Parser;

mod convert;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Application {
    #[arg(short, long)]
    directory: Option<PathBuf>,
    #[arg(short, long)]
    pattern: Option<String>,
}

pub fn main() -> anyhow::Result<()> {
    env_logger::init();
    let app = Application::parse();
    app.run()
}