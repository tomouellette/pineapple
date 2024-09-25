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
// use pineapple::index::JumpIndex;
// use pineapple::utils::progress_log;
// 
// fn main() -> () {
//     progress_log("Initializing JUMP download");
// 
//     let mut index = JumpIndex::init();
//     
//     let compound = Some("KYRVNWMVYQXFEU-UHFFFAOYSA-N".to_string());
//     index.query(None, None, None, None, None, compound);
// 
//     let n_queries = index.queries.len();
// 
//     progress_log(format!("Detected {} samples for downloading.", n_queries).as_str());
// 
//     index.download(Some("test_images/".to_string()), Some(6));
// 
//     progress_log("Download complete.");
// }
