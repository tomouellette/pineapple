use std::fs::File;
use std::path::Path;
use std::io::Write;
use tokio::time::{sleep, Duration};
use reqwest;
use reqwest::Client;

const CPG0016_ROOT: &str = "cellpainting-gallery";
const CPG0016_PATH: &str = "cpg0016-jump";

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_SECS: u64 = 2;
const REQUEST_TIMEOUT_SECS: u64 = 30;

/// A struct specifying the metadata for each jump cpg0016 image
#[derive(Debug, serde::Deserialize, Clone)]
pub struct JumpCpg0016Image {
    source: String,
    batch: String,
    plate: String,
    site: String,
    well: String,
    illum: String,
    filename: String,
    path: String,
    compound: String,
}

impl JumpCpg0016Image {
    pub fn check_source(&self, source: Option<String>) -> bool {
        match source {
            Some(s) => self.source == s,
            None => true,
        }
    }

    pub fn check_batch(&self, batch: Option<String>) -> bool {
        match batch {
            Some(b) => self.batch == b,
            None => true,
        }
    }

    pub fn check_plate(&self, plate: Option<String>) -> bool {
        match plate {
            Some(p) => self.plate == p,
            None => true,
        }
    }

    pub fn check_site(&self, site: Option<String>) -> bool {
        match site {
            Some(s) => self.site == s,
            None => true,
        }
    }

    pub fn check_well(&self, well: Option<String>) -> bool {
        match well {
            Some(w) => self.well == w,
            None => true,
        }
    }

    pub fn check_compound(&self, compound: Option<String>) -> bool {
        match compound {
            Some(c) => self.compound == c,
            None => true,
        }

    }

    /// Return a boolean indicating whether the image metadata matches the query
    ///
    /// # Arguments
    ///
    /// * `source` - Data generating center identifier
    /// * `batch` - Batch identifier for the plate
    /// * `plate` - Plate identifier
    /// * `site` - Number of sites per well
    /// * `well` - Well identifier
    /// * `compound` - Compound denoted by hashed InChIKey identifier
    pub fn filter(&self,
        source: Option<String>,
        batch: Option<String>,
        plate: Option<String>,
        site: Option<String>,
        well: Option<String>,
        compound: Option<String>,
    ) -> bool {
        if self.check_source(source) 
            && self.check_batch(batch)
            && self.check_plate(plate)
            && self.check_site(site)
            && self.check_well(well)
            && self.check_compound(compound) {
            true
        } else {
            false
        }
    }

    /// Download the image from the jump cpg0016 dataset
    ///
    /// # Arguments
    ///
    /// * `output` - Path to save downloaded images
    #[tokio::main]
    pub async fn download(&self, output: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let prefix = format!("s3://{}/{}/", CPG0016_ROOT, CPG0016_PATH);
        let path = self.path.strip_prefix(&prefix).unwrap();
        let url = format!("https://{}.s3.amazonaws.com/{}/{}{}", CPG0016_ROOT, CPG0016_PATH, path, self.filename);
        let output_name = output.join(&self.filename);

        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()?;

        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            attempt += 1;
            match client.get(&url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let content = response.bytes().await?;
                        let mut file = File::create(&output_name)?;
                        file.write_all(&content)?;
                        return Ok(());
                    } else {
                        println!("Failed download attempt {}. HTTP status {}", attempt, response.status());
                        println!("Response body: {}", response.text().await?);
                    }
                }
                Err(e) => {
                    println!("Failed download attempt {}[Error: {}]", attempt, e);
                }
            }

            if attempt < MAX_RETRIES {
                println!("Retrying image download in {} seconds...", RETRY_DELAY_SECS);
                sleep(Duration::from_secs(RETRY_DELAY_SECS)).await;
            }
        }

        Err("Failed to download file after multiple retries.".into())
    }
}
