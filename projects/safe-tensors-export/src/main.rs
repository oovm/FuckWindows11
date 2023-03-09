use std::path::PathBuf;

use clap::Parser;

mod convert;

/// Export ONNX models to SafeTensors format
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Application {
    /// Overwrite existing files
    overwrite: bool,
    /// Config file for the export
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// Pattern to match files
    #[arg(short, long)]
    pattern: Option<String>,
}

pub fn main() -> anyhow::Result<()> {
    env_logger::init();
    let app = Application::parse();
    app.run()
}