use clap::{Args, Subcommand};

use crate::args::jump_cpg0016::{JumpCpg0016Args, download_jump_cpg0016};

#[derive(Debug, Args)]
#[command(about = "Download bio-images from the command-line")]
#[command(args_conflicts_with_subcommands = true)]
#[command(arg_required_else_help = true)]
#[command(flatten_help = true)]
pub struct DownloadArgs {
    #[command(subcommand)]
    command: Option<DownloadCommands>,

    #[command(flatten)]
    jump_cpg0016: JumpCpg0016Args,
}

#[derive(Debug, Subcommand)]
enum DownloadCommands {
    JumpCpg0016(JumpCpg0016Args),
}

pub fn download(args: &DownloadArgs) {
    match args.command.as_ref().unwrap() {
        DownloadCommands::JumpCpg0016(images) => download_jump_cpg0016(images),
    }
}


