use clap::Args;
use crate::utils::progress_log;
use crate::table::jump_cpg0016::JumpCpg0016Table;

#[derive(Debug, Args)]
#[command(about = "Download images from the jump-cpg0016 dataset")]
pub struct JumpCpg0016Args {
    #[arg(long, help="Data generating center identifier")]
    pub source: Option<String>,

    #[arg(long, help="Batch identifier for the plate")]
    pub batch: Option<String>,

    #[arg(long, help="Plate identifier")]
    pub plate: Option<String>,

    #[arg(long, help="Number of sites per well")]
    pub site: Option<String>,

    #[arg(long, help="Well identifier")]
    pub well: Option<String>,

    #[arg(long, help="Compound denoted by hashed InChIKey identifier")]
    pub compound: Option<String>,

    #[arg(short, long, help="Path to save downloaded images")]
    pub output: Option<String>,

    #[arg(short, long, help="Number of threads to use for downloading")]
    pub threads: Option<usize>,

    #[arg(short, long, help="Download all images")]
    pub all: bool,
}

pub fn download_jump_cpg0016(args: &JumpCpg0016Args) {
    if args.output.is_none() {
        progress_log("Output directory not provided.");
        std::process::exit(1);
    }

    if args.source.is_none()
        && args.batch.is_none()
        && args.plate.is_none()
        && args.site.is_none()
        && args.well.is_none()
        && args.compound.is_none() 
        && !args.all
    {
        progress_log("No query parameters provided. To download all images, use the --all flag.");
        std::process::exit(1);
    }

    progress_log("Initializing jump-cpg0016 image download.");

    let mut table = JumpCpg0016Table::init();
    
    table.query(
        args.source.to_owned(),
        args.batch.to_owned(),
        args.plate.to_owned(),
        args.site.to_owned(),
        args.well.to_owned(),
        args.compound.to_owned(),
    );

    let n_queries = table.queries.len();

    progress_log(format!("Detected {} samples for downloading.", n_queries).as_str());

    table.download(args.output.to_owned(), args.threads.to_owned());
}
