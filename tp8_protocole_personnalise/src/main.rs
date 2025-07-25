fn main() {
    println!("TP8 - Protocole de Messagerie Personnalisé");
    println!("===========================================");
    println!();
    println!("Pour utiliser ce projet:");
    println!("1. Démarrer le serveur: cargo run --bin serveur");
    println!("2. Démarrer un client:  cargo run --bin client");
    println!();
    println!("Fonctionnalités implémentées:");
    println!("- Protocole de messagerie TCP avec format JSON");
    println!("- Chat multi-utilisateurs en temps réel");
    println!("- Commandes: /help, /list, /quit");
    println!("- Gestion des connexions/déconnexions");
    println!("- Authentification par nom d'utilisateur");
    println!("- Messages d'erreur et codes de statut");
}
