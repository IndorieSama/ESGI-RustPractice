use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;

const LOG_FILE_PATH: &str = "logs/server.log";
const SERVER_PORT: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log_writer = Arc::new(Mutex::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_FILE_PATH)
            .await?
    ));

    let listener = TcpListener::bind(SERVER_PORT).await?;
    println!("Serveur de journalisation démarré sur {}", SERVER_PORT);

    loop {
        let (socket, addr) = listener.accept().await?;
        let writer = Arc::clone(&log_writer);
        
        println!("Nouvelle connexion de {}", addr);
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, writer).await {
                eprintln!("Erreur lors du traitement du client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(
    socket: TcpStream,
    log_writer: Arc<Mutex<tokio::fs::File>>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(socket);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        
        if bytes_read == 0 {
            break;
        }

        let message = line.trim();
        if !message.is_empty() {
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
            let log_entry = format!("[{}] {}\n", timestamp, message);

            let mut writer = log_writer.lock().await;
            writer.write_all(log_entry.as_bytes()).await?;
            writer.flush().await?;
        }
    }

    Ok(())
}
