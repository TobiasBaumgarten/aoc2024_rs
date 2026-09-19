use reqwest::header::COOKIE;
use std::{error::Error, path::Path};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=.env");
    if let Err(e) = load() {
        println!("cargo::error={e}");
    }
}

fn load() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("aoc_input");
    std::fs::create_dir_all(&dest_path)?;
    let session = std::env::var("SESSION").expect("SESSION not set");
    for day in 1..25 {
        let url = format!("https://adventofcode.com/2024/day/{day}/input");
        let respond = reqwest::blocking::Client::new()
            .get(url)
            .header(COOKIE, format!("session={session}"))
            .send()?
            .error_for_status()?
            .text()?;
        let dest_file = dest_path.join(&format!("day_{:02}.txt", day));
        if dest_file.exists() {
            return Ok(()); // already fetched, don't hit the server again
        }
        std::fs::write(dest_file, respond)?;
    }
    Ok(())
}
