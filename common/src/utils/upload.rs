use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use tokio::fs::File as AsyncFile;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

async fn upload_chunk(file: &mut AsyncFile, chunk: Vec<u8>) -> io::Result<()> {
    file.write_all(&chunk).await?;
    Ok(())
}

async fn upload_file(file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    let file_size = path.metadata()?.len();

    let file = File::open(path)?;
    let mut file_reader = tokio::fs::File::from_std(file);
    let mut buffer = vec![0; CHUNK_SIZE];

    for chunk_number in 0..=(file_size / CHUNK_SIZE as u64) {
        let bytes_read = file_reader.read(&mut buffer).await?;
        if bytes_read == 0 {
            break; // End of file
        }

        let chunk = buffer[..bytes_read].to_vec();

        // Simulate uploading the chunk (replace this with actual upload logic)
        println!(
            "Uploading chunk {} of size {} bytes",
            chunk_number, bytes_read
        );

        // Simulate async upload
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        upload_chunk(&mut file_reader, chunk).await?;
    }

    Ok(())
}

#[tokio::main]
pub async fn upload(file_path: &str) -> io::Result<()> {
    upload_file(file_path).await?;
    Ok(())
}
