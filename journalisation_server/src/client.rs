use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, stdin, AsyncBufReadExt, BufReader};
use tokio::time::{Duration, Instant};

pub struct ClientConnection {
    pub stream: TcpStream,
    pub port: u16,
}

impl ClientConnection {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = TcpStream::connect(addr).await?;
        let port = stream.local_addr()?.port();
        Ok(ClientConnection { stream, port })
    }

    pub async fn send_message(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let message = format!("[Client:{}] {}\n", self.port, content);
        self.stream.write_all(message.as_bytes()).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn send_burst_messages(&mut self, count: u32, start_time: Instant, delay_ms: u64) -> Result<(), Box<dyn std::error::Error>> {
        for i in 1..=count {
            let content = format!("Message rapide {} - {}ms", i, start_time.elapsed().as_millis());
            self.send_message(&content).await?;
            tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        }
        Ok(())
    }

    pub async fn interactive_mode(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Tapez vos messages (Ctrl+C pour quitter):");
        
        let stdin = stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();

        loop {
            print!("> ");
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    self.send_message(line.trim()).await?;
                }
                Err(e) => {
                    eprintln!("Erreur de lecture: {}", e);
                    break;
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientConnection::connect("127.0.0.1:8080").await?;
    println!("Connecté depuis le port {}!", client.port);

    println!("Envoi de messages en rafale pour tester la concurrence...");
    let start = Instant::now();
    client.send_burst_messages(10, start, 10).await?;

    println!("Messages envoyés!");
    client.interactive_mode().await?;

    println!("Déconnexion...");
    Ok(())
}