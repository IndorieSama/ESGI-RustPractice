use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::Mutex;

// Chemin du fichier de log et port du serveur
const LOG_FILE_PATH: &str = "logs/server.log";
const SERVER_PORT: &str = "127.0.0.1:8080";

// Point d'entrée principal du serveur asynchrone.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Arc<Mutex<...>> pour permettre un accès concurrent sécurisé au fichier de log.
    let log_writer = Arc::new(Mutex::new( //Arc<Mutex<...>> pour permettre un accès concurrent sécurisé.
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_FILE_PATH)
            .await?
    ));

    // Démarre le serveur TCP sur l'adresse et le port spécifiés.
    let listener = TcpListener::bind(SERVER_PORT).await?;
    println!("Serveur de journalisation démarré sur {}", SERVER_PORT);

    // Boucle principale : accepte chaque client et lance une tâche dédiée.
    loop {
        let (socket, addr) = listener.accept().await?;
        let writer = Arc::clone(&log_writer);

        println!("Nouvelle connexion de {}", addr);

        // Chaque client est traité dans une tâche asynchrone indépendante.
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, writer).await {
                eprintln!("Erreur lors du traitement du client {}: {}", addr, e);
            }
        });
    }
}

// Gère une connexion client : lit, horodate et journalise chaque ligne reçue.
async fn handle_client(
    socket: TcpStream,
    log_writer: Arc<Mutex<tokio::fs::File>>
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(socket);
    let mut line = String::new();

    // Boucle de lecture des messages du client.
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        // Fin de connexion si le client ferme la socket.
        if bytes_read == 0 {
            break;
        }

        // Si la ligne reçue n'est pas vide, on la journalise.
        let message = line.trim();
        if !message.is_empty() {
            // Ajoute un horodatage UTC au message.
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
            let log_entry = format!("[{}] {}\n", timestamp, message);

            // Accès exclusif au fichier de log pour écrire l'entrée.
            let mut writer = log_writer.lock().await;
            writer.write_all(log_entry.as_bytes()).await?;
            writer.flush().await?;
        }
    }

    Ok(())
}
