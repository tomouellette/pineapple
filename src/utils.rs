// Utility functions for general use
use std::env;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use scraper::{Html, Selector};
use anyhow::{Result, Context, anyhow};
use reqwest::{Client, redirect::Policy};
use kdam::{tqdm, Bar, BarExt};
use dirs::home_dir;
use colored::*;
use chrono;

pub fn get_pineapple_cache() -> PathBuf {
    if let Ok(pineapple_cache) = env::var("PINEAPPLE_CACHE") {
        if !pineapple_cache.is_empty() {
            return PathBuf::from(pineapple_cache);
        }
    }

    if let Some(home) = home_dir() {
        return home.join(".pineapple_cache");
    }

    PathBuf::from("/.pineapple_cache")
}

pub fn progress_bar(n: usize, desc: &str) -> Bar {
    let pb = tqdm!(
        total = n,
        force_refresh = true,
        desc = progress_timestamp(desc),
        animation = "arrow",
        bar_format = "{desc suffix=' '}|{animation}|{count}/{total} [{percentage:.0}%] in {elapsed human=true} ({rate:.1}/s, eta: {remaining human=true})"
    );
    pb
}

pub fn progress_timestamp(desc: &str) -> String {
    let time = chrono::Local::now();
    let ymd = time.format("%Y-%m-%dT").to_string();
    let ymd = &ymd[..ymd.len() - 1];
    let hms = time.format("%H:%M:%S").to_string();
    let time = format!("{} | {}", ymd, hms);

    format!(
        "{} {} {} {} {} {}", 
        "[".bold(),
        time, 
        "|".bold(),
        "pineapple".truecolor(255, 45, 255).bold(), 
        "]".bold(),
        desc
    )
}

#[macro_export]
macro_rules! progress_log {
    ($desc:expr) => {
        println!("{}", pollen_core::helpers::progress_timestamp($desc));
    };
}

pub fn progress_log(desc: &str) {
    println!("{}", progress_timestamp(desc));
}

fn create_http_client() -> Result<Client> {
    Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .cookie_store(true)
        .redirect(Policy::limited(10))
        .build()
        .context("Failed to create HTTP client")
}

async fn handle_virus_scan_warning(client: &Client, url: &str) -> Result<String> {
    let resp = client.get(url)
        .send()
        .await
        .context("Failed to send initial request")?;

    let body = resp.text().await.context("Failed to get response body")?;
    
    if body.contains("Google Drive can't scan this file for viruses") {
        extract_download_url(&body)
    } else {
        Ok(url.to_string())
    }
}

fn extract_download_url(body: &str) -> Result<String> {
    let document = Html::parse_document(body);
    let form_selector = Selector::parse("form#download-form").unwrap();
    let input_selector = Selector::parse("input[name]").unwrap();

    let form = document.select(&form_selector).next()
        .ok_or_else(|| anyhow!("Download form not found in the HTML"))?;

    let action = form.value().attr("action")
        .ok_or_else(|| anyhow!("No form action found"))?;

    let params: Vec<(String, String)> = form.select(&input_selector)
        .filter_map(|input| {
            match (input.value().attr("name"), input.value().attr("value")) {
                (Some(name), Some(value)) => Some((name.to_string(), value.to_string())),
                _ => None,
            }
        })
        .collect();

    let query_string = params.into_iter()
        .map(|(name, value)| format!("{}={}", name, value))
        .collect::<Vec<_>>()
        .join("&");

    Ok(format!("{}?{}", action, query_string))
}

async fn download_file_with_progress(
    client: &Client,
    url: &str,
    output_dir: &Path,
    filename: &str,
    silent: bool,
) -> Result<()> {
    let mut resp = client.get(url)
        .send()
        .await
        .context("Failed to send download request")?;

    let total_size = resp.content_length().unwrap_or(0);
    let mut pb = progress_bar(total_size as usize, format!("Downloading {}", filename).as_str());

    let total_gigabytes = total_size as f64 / 1e9;

    if !silent {
        println!("{}", progress_timestamp(
            format!("Data table {} not detected. Downloading to cache ({:.2} GB)", filename, total_gigabytes).as_str()
        ));
    }

    if total_gigabytes == 0.0 {
        println!("Download could not be started. Please check connection and try again.");
        std::process::exit(1);
    }

    tokio::fs::create_dir_all(output_dir).await
        .context("Failed to create output directory")?;
    let filepath = output_dir.join(filename);
    let mut file = File::create(&filepath).await
        .context("Failed to create output file")?;

    while let Some(chunk) = resp.chunk().await.context("Failed to read chunk")? {
        file.write_all(&chunk).await.context("Failed to write chunk to file")?;

        if !silent {
            pb.update(chunk.len() as usize)?;
        }
    }

    progress_log("Data table cached. Proceeding to download data.");

    Ok(())
}

#[tokio::main]
pub async fn download_gdrive_file(
    file_id: &str,
    output_dir: &Path,
    filename: &str,
    silent: bool,
) -> Result<()> {
    let client = create_http_client()?;
    let initial_url = format!("https://drive.google.com/uc?id={}&export=download", file_id);
    let download_url = handle_virus_scan_warning(&client, &initial_url).await?;
    download_file_with_progress(&client, &download_url, output_dir, filename, silent).await?;
    Ok(())
}


