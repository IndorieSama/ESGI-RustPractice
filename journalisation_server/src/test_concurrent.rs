mod client;
use client::ClientConnection;
use tokio::time::Instant;

/// Client de test pour la concurrence : lance plusieurs connexions simultanées
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test de concurrence - Lancement de 10 clients simultanés...");
    
    let mut handles = vec![];
    let start_time = Instant::now();
    
    // Lancer 10 connexions simultanées
    for client_id in 1..=10 {
        let handle = tokio::spawn(async move {
            match test_client(client_id, start_time).await {
                Ok(_) => println!("Client {} terminé avec succès", client_id),
                Err(e) => eprintln!("Client {} erreur: {}", client_id, e),
            }
        });
        handles.push(handle);
    }
    
    // Attendre que tous les clients finissent
    for handle in handles {
        let _ = handle.await;
    }
    
    println!("Test de concurrence terminé! Vérifiez les logs du serveur.");
    Ok(())
}

async fn test_client(client_id: u32, start_time: Instant) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ClientConnection::connect("127.0.0.1:8080").await?;
    
    // Envoyer 5 messages rapidement
    for msg_id in 1..=5 {
        let elapsed = start_time.elapsed().as_millis();
        let content = format!("TestClient{} Message concurrent {} - {}ms", client_id, msg_id, elapsed);
        client.send_message(&content).await?;
        

        tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    }
    
    // Garder la connexion ouverte un peu pour voir les logs
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    Ok(())
}