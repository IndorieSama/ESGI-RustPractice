use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, stdin, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connexion au serveur de journalisation...");
    
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("Connecté! Tapez vos messages (Ctrl+C pour quitter):");

    let stdin = stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        print!("> ");
        line.clear();
        
        match reader.read_line(&mut line).await {
            Ok(0) => break,
            Ok(_) => {
                stream.write_all(line.as_bytes()).await?;
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