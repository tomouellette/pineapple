use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use rayon::prelude::*;
use csv::ReaderBuilder;
use kdam::TqdmParallelIterator;
use flate2::bufread::GzDecoder;
use crate::card::jump_cpg0016::JumpCpg0016Image;
use crate::utils::{progress_bar, get_pineapple_cache};
use crate::utils::download_gdrive_file;

const LOOKUP_TABLE: &str = "jump-cpg0016.csv.gz";
const LOOKUP_ID: &str = "1X-7Im3DYdgw1ITmIy_H4y1nclWDW8Uxh";

/// A struct for querying and downloading jump cpg0016 data
pub struct JumpCpg0016Table {
    pub table: String,
    pub queries: Vec<JumpCpg0016Image>,
}

impl JumpCpg0016Table {

    /// Initialize a new JumpCpg0016Table
    pub fn init() -> JumpCpg0016Table {
        let cache = get_pineapple_cache();
        let lookup_table = cache.join(LOOKUP_TABLE);
        if !lookup_table.exists() {
            let _ = download_gdrive_file(
                LOOKUP_ID,
                cache.as_path(),
                LOOKUP_TABLE,
                false,
            );
        }

        JumpCpg0016Table {
            table: lookup_table.to_str().unwrap().to_string(),
            queries: Vec::new(),
        }
    }

    /// Query a subset of the jump cpg0016 dataset based on metadata features
    ///
    /// # Arguments
    ///
    /// * `source` - Data generating center identifier
    /// * `batch` - Batch identifier for the plate
    /// * `plate` - Plate identifier
    /// * `site` - Number of sites per well
    /// * `well` - Well identifier
    /// * `compound` - Compound denoted by hashed InChIKey identifier
    ///
    /// # Example
    ///
    /// ```no_run
    /// let mut table = JumpCpg0016Table::init();
    /// let plate_id = Some("110000296354".to_string());
    /// table.query(None, None, plate_id, None, None, None);
    /// ```
    pub fn query(
        &mut self,
        source: Option<String>,
        batch: Option<String>,
        plate: Option<String>,
        site: Option<String>,
        well: Option<String>,
        compound: Option<String>,
    ) {
        let lookup = File::open(&self.table).unwrap_or_else(|err| {
            println!("Could not open the file: {}", err);
            std::process::exit(1);
        });

        let buffer = BufReader::new(lookup);

        let decoder = GzDecoder::new(buffer);

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(decoder);

        for row in reader.deserialize() {
            let image: JumpCpg0016Image = row.unwrap();
            let filter: bool = image.filter(
                source.clone(),
                batch.clone(),
                plate.clone(),
                site.clone(),
                well.clone(),
                compound.clone(),
            );

            match filter {
                true => self.queries.push(image),
                false => continue,
            }
        }
    }

    /// Download all or a subset of the jump cpg0016 dataset
    ///
    /// # Arguments
    ///
    /// * `output` - Output directory for downloaded images
    /// * `threads` - Number of threads to use for downloading
    ///
    /// # Example
    ///
    /// ```no_run
    /// let mut table = JumpCpg0016Table::init();
    /// let output = Some("output".to_string());
    /// table.download(output, Some(4));
    /// ```
    pub fn download(&self, output: Option<String>, threads: Option<usize>) {
        let output_path = Path::new(output.as_deref().unwrap());

        std::fs::create_dir_all(&output_path).unwrap_or_else(|err| {
            println!("Could not create the directory: {}", err);
            std::process::exit(1);
        });

        if let Some(threads) = threads {
            if threads < 1 {
                println!("Threads must be set to a positive integer");
                std::process::exit(1);
            }

            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global()
                .unwrap();
        }
        
        let pb = progress_bar(self.queries.len(), "Downloading jump-cpg0016 images");

        self.queries
            .par_iter()
            .tqdm_with_bar(pb)
            .for_each(|query| {
                query.download(&output_path).unwrap_or_else(|err| {
                println!("Could not download file: {}", err);
                std::process::exit(1);
            });
        });
    }
}