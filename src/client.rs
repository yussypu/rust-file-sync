use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use tokio::sync::mpsc;
use notify::Config;
use notify::Event;
use crate::encryption::encrypt;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::fs;
use std::time::Instant;
use std::collections::HashSet; 
use colored::*;  
use indicatif::{ProgressBar, ProgressStyle}; 

pub async fn start_client() {
    if let Err(e) = client_main().await {
        eprintln!("{}", "❌ Client error:".red());
        eprintln!("{:?}", e);
    }
}

async fn client_main() -> Result<(), Box<dyn std::error::Error>> { 
    println!("{}", "🚀 Watching ./sync_folder for changes...".cyan());

    // Sync channel for notify
    let (tx, mut rx): (mpsc::Sender<Event>, mpsc::Receiver<Event>) = mpsc::channel(100); // Specify the type

    // Create file system watcher
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                tx.blocking_send(event).unwrap(); // Send events to the Tokio channel
            }
        },
        Config::default()
    )?;

    watcher.watch(Path::new("./sync_folder"), RecursiveMode::Recursive)?;

    // Timing benchmark
    let mut total_time = std::time::Duration::ZERO;
    let mut file_count = 0;
    let benchmark_target = 500; // Total files to be synced

    // HashSet to track processed files
    let mut processed_files = HashSet::new(); 

    // Create the progress bar
    let pb = ProgressBar::new(benchmark_target as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40}] {percent:3}%" ) // Customize the progress bar
        .progress_chars("=>-"));

    // Listen for events
    while let Some(event) = rx.recv().await {
        for path in event.paths {
            if path.is_file() && !processed_files.contains(&path) { // Check if file has already been processed
                println!("📂 Detected change at {:?}", path);

                let start = Instant::now();
                send_file(&path).await?;
                let elapsed = start.elapsed();
                println!("⏱️ Time from detection to send: {:.2?}", elapsed);

                total_time += elapsed;  // Accumulate the total time
                file_count += 1;  // Increment the file counter

                // Log file count progress
                println!("File count: {}", file_count);

                // Only print the average after all files are processed
                if file_count == benchmark_target {
                    let avg = total_time / file_count as u32;  // Calculate average time per file
                    println!("\n🏁 Benchmark Completed:");
                    println!("{}", format!("➡️  Synced {} files.", file_count).green());
                    println!("{}", format!("➡️  Average time per file: {:.2?}", avg).green());  // Print the average time
                    return Ok(()); // End the function after printing the result
                }

                // Add file to the set to prevent further processing
                processed_files.insert(path.clone());

                // Update the progress bar
                pb.inc(1);
            }
        }
    }

    Ok(())
}
async fn send_file(path: &Path) -> Result<(), Box<dyn std::error::Error>> { 
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let file_contents = fs::read(path)?;

    // Encrypt
    let (nonce, ciphertext) = encrypt(&file_contents);

    // Send file name length
    stream.write_u16(file_name.len() as u16).await?;
    stream.write_all(file_name.as_bytes()).await?;

    // Send nonce
    stream.write_all(&nonce).await?;

    // Send ciphertext
    stream.write_all(&ciphertext).await?;

    println!("{}", format!("✅ Automatically sent file: {:?}", file_name).green());
    Ok(())
}