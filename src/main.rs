use clap::{Parser, Subcommand};
use pineapple::download;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Download(download::DownloadArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Download(download_args)) => download::download(download_args),
        None => {}
    }
}
