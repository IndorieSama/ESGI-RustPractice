use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, stdin, AsyncBufReadExt, BufReader};

/// Client simple : se connecte au serveur, lit l'entrée utilisateur et envoie chaque ligne.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connexion au serveur
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let local_addr = stream.local_addr()?;
    println!("Connecté depuis le port {}! Tapez vos messages (Ctrl+C pour quitter):", local_addr.port());

    // Préparation de la lecture utilisateur
    let stdin = stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    // Boucle principale : lecture et envoi
    loop {
        print!("> ");
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                let message = format!("[Client:{}] {}", local_addr.port(), line.trim());
                stream.write_all(format!("{}\n", message).as_bytes()).await?;
                stream.flush().await?;
            }
            Err(e) => {
                eprintln!("Erreur de lecture: {}", e);
                break;
            }
        }
    }

    println!("Déconnexion...");
    Ok(())
}