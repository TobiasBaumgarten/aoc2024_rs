use reqwest::header::COOKIE;
use std::{error::Error, path::Path};
const YEAR: &str = "2024";
const BASE_URL: &str = "https://adventofcode.com/";

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=.env");
    if let Err(e) = download_input() {
        println!("cargo::error={e}");
    }
    if let Err(e) = download_challenges() {
        println!("cargo::error={e}");
    }
}

fn download_challenges() -> Result<(), Box<dyn Error>> {
    let out_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(".challenges/src");
    if dest_path.exists() {
        return Ok(());
    }
    std::fs::create_dir_all(&dest_path)?;
    for day in 1..25 {
        let url = format!("{BASE_URL}{YEAR}/day/{day}");
        let response = reqwest::blocking::Client::new()
            .get(url)
            .send()?
            .error_for_status()?
            .text()?;
        let md = html2md::rewrite_html(&response, true);
        let dest_file = dest_path.join(format!("day_{:02}.rs.md", day));
        std::fs::write(dest_file, md)?;
    }
    Ok(())
}

/// Downloads the input from the advent of cade site
fn download_input() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let out_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(".aoc_input/src");
    if dest_path.exists() {
        return Ok(());
    }
    std::fs::create_dir_all(&dest_path)?;
    let session = std::env::var("SESSION").expect("SESSION not set");
    for day in 1..25 {
        let url = format!("{BASE_URL}{YEAR}/day/{day}/input");
        let respond = reqwest::blocking::Client::new()
            .get(url)
            .header(COOKIE, format!("session={session}"))
            .send()?
            .error_for_status()?
            .text()?;
        let dest_file = dest_path.join(format!("day_{:02}.rs.txt", day));
        if dest_file.exists() {
            return Ok(()); // already fetched, don't hit the server again
        }
        std::fs::write(dest_file, respond)?;
    }
    Ok(())
}
