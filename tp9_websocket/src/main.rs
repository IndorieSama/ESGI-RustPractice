fn main() {
    println!("TP9 - Serveur et Client WebSocket");
    println!("==================================");
    println!();
    println!("Pour utiliser ce projet:");
    println!("1. Démarrer le serveur: cargo run --bin serveur_websocket");
    println!("2. Démarrer un client:  cargo run --bin client_websocket");
    println!();
    println!("Fonctionnalités implémentées:");
    println!("- Serveur WebSocket multi-connexions avec tokio-tungstenite");
    println!("- Chat en temps réel avec diffusion des messages");
    println!("- Gestion des messages texte et binaires");
    println!("- Handshake WebSocket automatique");
    println!("- Communication full-duplex persistante");
    println!("- Commandes: /help, /users, /stats, /ping, /file, /quit");
    println!("- Simulation d'envoi de fichiers binaires");
    println!("- Validation des noms d'utilisateur");
    println!("- Gestion robuste des erreurs et déconnexions");
}
