use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use std::fs;
use std::path::PathBuf;
use std::io::Error;
use crate::encryption::decrypt;

pub async fn start_server() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("🚀 Server listening on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("✅ Client connected!");

        // Read file name length (u16)
        let name_len = socket.read_u16().await?;

        // Read file name bytes
        let mut name_buf = vec![0u8; name_len as usize];
        socket.read_exact(&mut name_buf).await?;
        let file_name = String::from_utf8(name_buf).unwrap();

        // Prepare file path
        let mut file_path = PathBuf::from("synced_folder");
        fs::create_dir_all(&file_path)?;
        file_path.push(file_name);

        // Read nonce (12 bytes)
        let mut nonce = [0u8; 12];
        socket.read_exact(&mut nonce).await?;

        // Read ciphertext (rest of the stream)
        let mut ciphertext = Vec::new();
        socket.read_to_end(&mut ciphertext).await?;

        // Decrypt
        let plaintext = decrypt(&nonce, &ciphertext);

        // Save decrypted contents
        fs::write(&file_path, &plaintext)?;

        println!("📥 File received, decrypted, and saved!");
    }
}
