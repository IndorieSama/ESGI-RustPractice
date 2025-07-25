use futures_util::{SinkExt, StreamExt};
use std::io::{self, Write};
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tp9_websocket::{MessageWebSocket, TypeMessageWs, utils};

/// Client WebSocket pour chat en temps réel
pub struct ClientWebSocket {
    nom_utilisateur: String,
    url_serveur: String,
    connecte: bool,
}

impl ClientWebSocket {
    /// Crée un nouveau client WebSocket
    pub fn new(nom_utilisateur: String, url_serveur: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // Valide le nom d'utilisateur
        utils::valider_nom_utilisateur(&nom_utilisateur)?;
        
        Ok(ClientWebSocket {
            nom_utilisateur,
            url_serveur,
            connecte: false,
        })
    }

    /// Se connecte au serveur WebSocket
    pub async fn connecter(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Connexion au serveur WebSocket {}...", self.url_serveur);
        
        // Établit la connexion WebSocket
        let (ws_stream, _) = connect_async(&self.url_serveur).await?;
        println!("Connexion WebSocket établie !");
        
        // Divise le stream en lecture et écriture
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        
        // Envoie le message de connexion
        let message_connexion = MessageWebSocket::nouvelle_connexion(self.nom_utilisateur.clone());
        let msg_ws = message_connexion.vers_message_websocket()?;
        ws_sender.send(msg_ws).await?;
        
        println!("Message de connexion envoyé, en attente de confirmation...");
        
        // Attend la confirmation
        match timeout(Duration::from_secs(10), ws_receiver.next()).await {
            Ok(Some(Ok(message))) => {
                if let Ok(message_ws) = MessageWebSocket::depuis_message_websocket(message) {
                    if message_ws.type_message == TypeMessageWs::Notification {
                        println!("Connexion confirmée: {}", 
                            message_ws.contenu.unwrap_or("Connexion réussie".to_string()));
                        self.connecte = true;
                    } else {
                        return Err("Réponse inattendue du serveur".into());
                    }
                } else {
                    return Err("Impossible de parser la réponse du serveur".into());
                }
            }
            Ok(Some(Err(e))) => {
                return Err(format!("Erreur WebSocket: {}", e).into());
            }
            Ok(None) => {
                return Err("Connexion fermée par le serveur".into());
            }
            Err(_) => {
                return Err("Timeout lors de la connexion".into());
            }
        }
        
        // Lance la session interactive
        self.demarrer_session_interactive(ws_sender, ws_receiver).await?;
        
        Ok(())
    }

    /// Démarre la session interactive de chat
    async fn demarrer_session_interactive(
        &mut self,
        mut ws_sender: futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>,
        mut ws_receiver: futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("\n=== Chat WebSocket Démarré ===");
        println!("Tapez vos messages et appuyez sur Entrée");
        println!("Commandes spéciales:");
        println!("  /help - Affiche l'aide");
        println!("  /users - Liste des utilisateurs");
        println!("  /stats - Statistiques du serveur");
        println!("  /ping - Teste la connexion");
        println!("  /file <nom> - Simule l'envoi d'un fichier");
        println!("  /quit - Se déconnecter");
        println!("================================\n");

        let nom_clone = self.nom_utilisateur.clone();
        
        // Lance la tâche d'écoute des messages du serveur
        tokio::spawn(async move {
            Self::ecouter_serveur(ws_receiver, &nom_clone).await;
        });

        // Boucle principale d'envoi de messages
        self.boucle_envoi_messages(&mut ws_sender).await?;
        
        Ok(())
    }

    /// Boucle principale pour l'envoi de messages
    async fn boucle_envoi_messages(
        &mut self,
        ws_sender: &mut futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            // Demande à l'utilisateur de saisir un message
            print!("> ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let message_text = input.trim();
                    
                    if message_text.is_empty() {
                        continue;
                    }
                    
                    // Traite la commande /quit localement
                    if message_text.eq_ignore_ascii_case("/quit") {
                        println!("Déconnexion...");
                        let deconnexion = MessageWebSocket::nouvelle_deconnexion(self.nom_utilisateur.clone());
                        let msg_ws = deconnexion.vers_message_websocket()?;
                        let _ = ws_sender.send(msg_ws).await;
                        break;
                    }
                    
                    // Traite la commande /file spéciale
                    if message_text.starts_with("/file ") {
                        let nom_fichier = message_text.strip_prefix("/file ").unwrap_or("test.txt");
                        if let Err(e) = self.envoyer_fichier_simule(ws_sender, nom_fichier).await {
                            eprintln!("Erreur d'envoi de fichier: {}", e);
                        }
                        continue;
                    }
                    
                    // Crée et envoie le message
                    let message = MessageWebSocket::nouveau_chat(self.nom_utilisateur.clone(), message_text.to_string());
                    
                    match message.vers_message_websocket() {
                        Ok(msg_ws) => {
                            if let Err(e) = ws_sender.send(msg_ws).await {
                                eprintln!("Erreur d'envoi: {}", e);
                                break;
                            }
                        }
                        Err(e) => {
                            eprintln!("Erreur de sérialisation: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Erreur de lecture: {}", e);
                    break;
                }
            }
        }
        
        self.connecte = false;
        Ok(())
    }

    /// Simule l'envoi d'un fichier binaire
    async fn envoyer_fichier_simule(
        &self,
        ws_sender: &mut futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, Message>,
        nom_fichier: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Simulation d'envoi du fichier '{}'...", nom_fichier);
        
        // Crée des données binaires simulées
        let donnees_simulees = match nom_fichier.split('.').last() {
            Some("txt") => b"Ceci est le contenu d'un fichier texte simulate.".to_vec(),
            Some("jpg") | Some("png") => {
                // Simule une en-tête d'image
                let mut donnees = vec![0xFF, 0xD8, 0xFF, 0xE0]; // En-tête JPEG
                donnees.extend_from_slice(b"Donnees d'image simulees...");
                donnees.extend(vec![0u8; 100]); // Padding pour simuler plus de données
                donnees
            }
            _ => {
                // Fichier générique
                let mut donnees = b"Fichier binaire simule: ".to_vec();
                donnees.extend_from_slice(nom_fichier.as_bytes());
                donnees.extend(vec![42u8; 50]); // Données binaires arbitraires
                donnees
            }
        };
        
        // Crée le message binaire
        let message_binaire = MessageWebSocket::nouveau_binaire(
            self.nom_utilisateur.clone(),
            donnees_simulees,
            Some(nom_fichier.to_string()),
        );
        
        let msg_ws = message_binaire.vers_message_websocket()?;
        ws_sender.send(msg_ws).await?;
        
        println!("Fichier '{}' envoyé ({} bytes)", nom_fichier, 
            message_binaire.donnees_binaires.as_ref().map(|d| d.len()).unwrap_or(0));
        
        Ok(())
    }

    /// Écoute les messages du serveur
    async fn ecouter_serveur(
        mut ws_receiver: futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
        nom_utilisateur: &str,
    ) {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(message) => {
                    if let Ok(message_ws) = MessageWebSocket::depuis_message_websocket(message) {
                        Self::traiter_message_recu(message_ws, nom_utilisateur).await;
                    }
                }
                Err(e) => {
                    eprintln!("\nErreur WebSocket: {}", e);
                    break;
                }
            }
        }
        println!("\nConnexion WebSocket fermée");
    }

    /// Traite un message reçu du serveur
    async fn traiter_message_recu(message: MessageWebSocket, nom_utilisateur: &str) {
        match message.type_message {
            TypeMessageWs::Chat => {
                // Affiche les messages de chat, sauf ceux envoyés par nous-mêmes
                if let Some(ref expediteur) = message.utilisateur {
                    if expediteur != nom_utilisateur {
                        println!("\r{}", message);
                        print!("> ");
                        let _ = io::stdout().flush();
                    }
                } else {
                    // Messages du serveur
                    println!("\r{}", message);
                    print!("> ");
                    let _ = io::stdout().flush();
                }
            }
            TypeMessageWs::Binaire => {
                // Affiche les notifications de fichiers binaires
                if let Some(ref expediteur) = message.utilisateur {
                    if expediteur != nom_utilisateur {
                        println!("\r{}", message);
                        print!("> ");
                        let _ = io::stdout().flush();
                    }
                }
            }
            TypeMessageWs::Notification => {
                // Affiche toutes les notifications système
                println!("\r{}", message);
                print!("> ");
                let _ = io::stdout().flush();
            }
            TypeMessageWs::Connexion | TypeMessageWs::Deconnexion => {
                // Affiche les notifications de connexion/déconnexion
                println!("\r{}", message);
                print!("> ");
                let _ = io::stdout().flush();
            }
            TypeMessageWs::ListeUtilisateurs => {
                if let Some(metadonnees) = message.metadonnees {
                    if let Ok(liste) = serde_json::from_value::<Vec<String>>(metadonnees) {
                        println!("\rUtilisateurs connectés ({}): {}", liste.len(), liste.join(", "));
                        print!("> ");
                        let _ = io::stdout().flush();
                    }
                }
            }
            TypeMessageWs::Ping => {
                println!("\rPING reçu du serveur");
                print!("> ");
                let _ = io::stdout().flush();
            }
            TypeMessageWs::Pong => {
                println!("\rPONG reçu du serveur");
                print!("> ");
                let _ = io::stdout().flush();
            }
            _ => {
                // Ignore les autres types de messages
            }
        }
    }

    /// Demande le nom d'utilisateur à l'utilisateur
    fn demander_nom_utilisateur() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        loop {
            print!("Entrez votre nom d'utilisateur (sans espaces): ");
            io::stdout().flush()?;
            
            let mut nom = String::new();
            io::stdin().read_line(&mut nom)?;
            let nom = nom.trim().to_string();
            
            match utils::valider_nom_utilisateur(&nom) {
                Ok(()) => return Ok(nom),
                Err(e) => println!("Erreur: {}", e),
            }
        }
    }

    /// Demande l'URL du serveur
    fn demander_url_serveur() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        print!("URL du serveur WebSocket (par défaut: ws://127.0.0.1:9001): ");
        io::stdout().flush()?;
        
        let mut url = String::new();
        io::stdin().read_line(&mut url)?;
        let url = url.trim();
        
        if url.is_empty() {
            Ok("ws://127.0.0.1:9001".to_string())
        } else {
            // Valide basiquement l'URL
            if url.starts_with("ws://") || url.starts_with("wss://") {
                Ok(url.to_string())
            } else {
                Ok(format!("ws://{}", url))
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Client WebSocket pour Chat en Temps Réel ===");
    
    // Demande les informations de connexion
    let nom_utilisateur = ClientWebSocket::demander_nom_utilisateur()?;
    let url_serveur = ClientWebSocket::demander_url_serveur()?;
    
    // Crée et connecte le client
    let mut client = ClientWebSocket::new(nom_utilisateur, url_serveur)?;
    
    match client.connecter().await {
        Ok(()) => {
            println!("Session terminée.");
        }
        Err(e) => {
            eprintln!("Erreur: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("Au revoir!");
    Ok(())
}