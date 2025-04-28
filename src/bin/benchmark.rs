use std::fs::{self, File};
use std::io::Write;
use std::time::Duration;
use std::path::Path;

#[tokio::main]
async fn main() {
    println!("Starting benchmark...");

    let sync_folder = Path::new("./sync_folder");

    if sync_folder.exists() {
        fs::remove_dir_all(sync_folder).unwrap(); // Deletes the entire folder
        fs::create_dir_all(sync_folder).unwrap(); // Recreate it empty
    } else {
        fs::create_dir_all(sync_folder).unwrap(); // Create if folder doesn't exist
    }

    // Clean old files first
    if sync_folder.exists() {
        fs::remove_dir_all(sync_folder).unwrap();
    }
    fs::create_dir_all(sync_folder).unwrap();

    // Create 500 test files
    println!("Creating 5000 test files...");
    for i in 0..5000 {
        let file_path = sync_folder.join(format!("test_file_{}.txt", i));
        let mut file = File::create(file_path).unwrap();
        writeln!(file, "This is test file number {}", i).unwrap();
    }

    println!("Waiting for file changes to be picked up...");
    let wait_time = Duration::from_secs(5);
    tokio::time::sleep(wait_time).await; // give your main watcher some time

    println!("Benchmark finished!");
    println!("✅ Created 5000 files inside ./sync_folder.");
    println!("➡️  Watch your main watcher output for auto-sent files and timing now.");
    println!("(You can now calculate average time by watching 500 sends!)");
}
