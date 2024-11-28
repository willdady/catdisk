use std::path::Path;
use std::process::exit;

use anyhow::Result;
use clap::Parser;
use file::FileObj;

mod file;
mod fs_utils;
mod sql;

/// Recursively walks a directory writing file metadata to an SQLite database
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output database file name
    file: String,
    /// Path containing files to recursively index
    path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let c = sql::Client::new(&args.file).await?;

    let path = Path::new(&args.path);
    if !path.is_dir() {
        eprintln!("Error: Path is not a valid directory");
        exit(1);
    }

    let paths = fs_utils::walk_directory(path)?;
    for path in paths {
        let fo: FileObj = path.into();
        c.insert(&fo).await?;
    }

    c.close().await?;

    Ok(())
}
