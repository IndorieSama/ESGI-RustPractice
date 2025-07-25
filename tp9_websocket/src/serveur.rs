use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};
use tp9_websocket::{MessageWebSocket, TypeMessageWs, utils};

/// Structure représentant un utilisateur connecté via WebSocket
#[derive(Debug, Clone)]
struct UtilisateurWebSocket {
    nom: String,
    adresse: SocketAddr,
    id_session: String,
}

/// État partagé du serveur WebSocket
#[derive(Debug)]
struct EtatServeurWs {
    /// Liste des utilisateurs connectés
    utilisateurs: HashMap<String, UtilisateurWebSocket>,
    /// Canal de diffusion pour tous les messages
    diffuseur: broadcast::Sender<MessageWebSocket>,
    /// Compteur de connexions
    compteur_connexions: u64,
}

impl EtatServeurWs {
    fn new() -> (Self, broadcast::Receiver<MessageWebSocket>) {
        let (diffuseur, recepteur) = broadcast::channel(1000);
        let etat = EtatServeurWs {
            utilisateurs: HashMap::new(),
            diffuseur,
            compteur_connexions: 0,
        };
        (etat, recepteur)
    }

    /// Ajoute un nouvel utilisateur
    fn ajouter_utilisateur(&mut self, nom: String, adresse: SocketAddr) -> Result<String, String> {
        // Valide le nom d'utilisateur  
        if let Err(e) = utils::valider_nom_utilisateur(&nom) {
            return Err(e);
        }

        if self.utilisateurs.contains_key(&nom) {
            return Err(format!("L'utilisateur '{}' est déjà connecté", nom));
        }

        let id_session = utils::generer_id_session();
        let utilisateur = UtilisateurWebSocket {
            nom: nom.clone(),
            adresse,
            id_session: id_session.clone(),
        };

        self.utilisateurs.insert(nom.clone(), utilisateur);
        self.compteur_connexions += 1;

        // Diffuse un message de connexion
        let message_connexion = MessageWebSocket::nouvelle_notification(
            format!("{} a rejoint le chat ({} utilisateurs connectés)", nom, self.utilisateurs.len()),
        );
        let _ = self.diffuseur.send(message_connexion);

        println!("Utilisateur '{}' connecté depuis {} (session: {})", nom, adresse, id_session);
        Ok(id_session)
    }

    /// Supprime un utilisateur
    fn supprimer_utilisateur(&mut self, nom: &str) {
        if let Some(utilisateur) = self.utilisateurs.remove(nom) {
            // Diffuse un message de déconnexion
            let message_deconnexion = MessageWebSocket::nouvelle_deconnexion(nom.to_string());
            let _ = self.diffuseur.send(message_deconnexion);

            println!("Utilisateur '{}' déconnecté (session: {})", nom, utilisateur.id_session);
        }
    }

    /// Obtient la liste des utilisateurs connectés
    fn obtenir_liste_utilisateurs(&self) -> Vec<String> {
        self.utilisateurs.keys().cloned().collect()
    }

    /// Diffuse un message à tous les clients connectés
    fn diffuser_message(&self, message: MessageWebSocket) {
        let _ = self.diffuseur.send(message);
    }

    /// Obtient les statistiques du serveur
    fn obtenir_statistiques(&self) -> serde_json::Value {
        serde_json::json!({
            "utilisateurs_connectes": self.utilisateurs.len(),
            "total_connexions": self.compteur_connexions,
            "utilisateurs": self.obtenir_liste_utilisateurs()
        })
    }
}

/// Serveur WebSocket pour chat en temps réel
pub struct ServeurWebSocket {
    listener: TcpListener,
    etat: Arc<Mutex<EtatServeurWs>>,
}

impl ServeurWebSocket {
    /// Crée un nouveau serveur WebSocket
    pub async fn new(adresse: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(adresse).await?;
        let (etat_serveur, _) = EtatServeurWs::new();
        let etat = Arc::new(Mutex::new(etat_serveur));

        println!("Serveur WebSocket démarré sur {}", adresse);
        println!("Connectez-vous via: ws://{}", adresse.replace("0.0.0.0", "127.0.0.1"));
        println!("En attente de connexions...");

        Ok(ServeurWebSocket { listener, etat })
    }

    /// Démarre le serveur et écoute les connexions
    pub async fn demarrer(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            match self.listener.accept().await {
                Ok((stream, adresse)) => {
                    println!("Nouvelle connexion TCP depuis {}", adresse);
                    let etat = Arc::clone(&self.etat);

                    // Traite chaque connexion WebSocket dans une tâche séparée
                    tokio::spawn(async move {
                        if let Err(e) = Self::gerer_connexion_websocket(stream, adresse, etat).await {
                            eprintln!("Erreur avec la connexion WebSocket {}: {}", adresse, e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Erreur d'acceptation de connexion TCP: {}", e);
                }
            }
        }
    }

    /// Gère une connexion WebSocket
    async fn gerer_connexion_websocket(
        stream: TcpStream,
        adresse: SocketAddr,
        etat: Arc<Mutex<EtatServeurWs>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Effectue le handshake WebSocket
        let ws_stream = accept_async(stream).await?;
        println!("Handshake WebSocket réussi avec {}", adresse);

        // Divise le stream en lecture et écriture
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Variable pour stocker le nom d'utilisateur une fois connecté
        let mut nom_utilisateur: Option<String> = None;
        let mut recepteur_diffusion: Option<broadcast::Receiver<MessageWebSocket>> = None;

        // Attend le message de connexion initial
        if let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(message) => {
                    if let Ok(message_ws) = MessageWebSocket::depuis_message_websocket(message) {
                        if message_ws.type_message == TypeMessageWs::Connexion {
                            if let Some(nom) = message_ws.utilisateur {
                                // Tente d'ajouter l'utilisateur
                                let mut etat_lock = etat.lock().await;
                                match etat_lock.ajouter_utilisateur(nom.clone(), adresse) {
                                    Ok(id_session) => {
                                        nom_utilisateur = Some(nom.clone());
                                        
                                        // Crée un récepteur pour la diffusion
                                        recepteur_diffusion = Some(etat_lock.diffuseur.subscribe());
                                        
                                        // Envoie une confirmation de connexion
                                        let confirmation = MessageWebSocket::nouvelle_notification(
                                            format!("Bienvenue {} ! Vous êtes connecté au chat WebSocket.", nom)
                                        );
                                        
                                        if let Ok(msg_ws) = confirmation.vers_message_websocket() {
                                            let _ = ws_sender.send(msg_ws).await;
                                        }
                                        
                                        println!("Utilisateur '{}' authentifié (session: {})", nom, id_session);
                                    }
                                    Err(erreur) => {
                                        // Envoie l'erreur et ferme la connexion
                                        let msg_erreur = MessageWebSocket::nouvelle_notification(
                                            format!("Erreur de connexion: {}", erreur)
                                        );
                                        if let Ok(msg_ws) = msg_erreur.vers_message_websocket() {
                                            let _ = ws_sender.send(msg_ws).await;
                                        }
                                        return Ok(());
                                    }
                                }
                            } else {
                                let msg_erreur = MessageWebSocket::nouvelle_notification(
                                    "Nom d'utilisateur requis pour la connexion".to_string()
                                );
                                if let Ok(msg_ws) = msg_erreur.vers_message_websocket() {
                                    let _ = ws_sender.send(msg_ws).await;
                                }
                                return Ok(());
                            }
                        } else {
                            let msg_erreur = MessageWebSocket::nouvelle_notification(
                                "Connexion requise avant l'envoi de messages".to_string()
                            );
                            if let Ok(msg_ws) = msg_erreur.vers_message_websocket() {
                                let _ = ws_sender.send(msg_ws).await;
                            }
                            return Ok(());
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Erreur lors de la réception du message initial: {}", e);
                    return Ok(());
                }
            }
        } else {
            println!("Connexion fermée avant l'authentification");
            return Ok(());
        }

        // À partir d'ici, l'utilisateur est authentifié
        if let (Some(nom), Some(mut recepteur)) = (nom_utilisateur.as_ref(), recepteur_diffusion) {
            let nom_clone = nom.clone();
            let etat_clone = Arc::clone(&etat);

            // Lance la tâche de diffusion des messages vers ce client
            let nom_diffusion = nom_clone.clone();
            tokio::spawn(async move {
                while let Ok(message) = recepteur.recv().await {
                    // Ne renvoie pas les messages de ce client à lui-même (sauf notifications système)
                    if let Some(ref expediteur) = message.utilisateur {
                        if expediteur == &nom_diffusion && message.type_message != TypeMessageWs::Notification {
                            continue;
                        }
                    }

                    if let Ok(msg_ws) = message.vers_message_websocket() {
                        if ws_sender.send(msg_ws).await.is_err() {
                            break; // Connexion fermée
                        }
                    }
                }
            });

            // Traite les messages entrants du client
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(message) => {
                        if let Ok(message_ws) = MessageWebSocket::depuis_message_websocket(message) {
                            if let Err(e) = Self::traiter_message_client(&nom_clone, &etat_clone, message_ws).await {
                                eprintln!("Erreur lors du traitement du message de {}: {}", nom_clone, e);
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Erreur WebSocket pour {}: {}", nom_clone, e);
                        break;
                    }
                }
            }
        }

        // Nettoyage: supprime l'utilisateur s'il était connecté
        if let Some(nom) = nom_utilisateur {
            let mut etat_lock = etat.lock().await;
            etat_lock.supprimer_utilisateur(&nom);
        }

        println!("Connexion WebSocket fermée pour {}", adresse);
        Ok(())
    }

    /// Traite un message spécifique d'un client
    async fn traiter_message_client(
        nom_utilisateur: &str,
        etat: &Arc<Mutex<EtatServeurWs>>,
        message: MessageWebSocket,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match message.type_message {
            TypeMessageWs::Chat => {
                if let Some(contenu) = message.contenu {
                    // Traite les commandes spéciales
                    if contenu.starts_with('/') {
                        Self::traiter_commande(nom_utilisateur, etat, &contenu).await?;
                    } else {
                        // Diffuse le message de chat à tous les clients
                        let message_diffusion = MessageWebSocket::nouveau_chat(
                            nom_utilisateur.to_string(),
                            contenu,
                        );
                        let etat_lock = etat.lock().await;
                        etat_lock.diffuser_message(message_diffusion);
                    }
                }
            }
            TypeMessageWs::Binaire => {
                if let Some(donnees) = message.donnees_binaires {
                    // Diffuse le message binaire
                    let nom_fichier = message.metadonnees
                        .as_ref()
                        .and_then(|m| m.get("nom_fichier"))
                        .and_then(|n| n.as_str())
                        .map(|s| s.to_string());
                    
                    let message_binaire = MessageWebSocket::nouveau_binaire(
                        nom_utilisateur.to_string(),
                        donnees,
                        nom_fichier,
                    );
                    let etat_lock = etat.lock().await;
                    etat_lock.diffuser_message(message_binaire);
                }
            }
            TypeMessageWs::DemandeUtilisateurs => {
                let etat_lock = etat.lock().await;
                let liste = etat_lock.obtenir_liste_utilisateurs();
                let reponse = MessageWebSocket::nouvelle_liste_utilisateurs(liste);
                etat_lock.diffuser_message(reponse);
            }
            TypeMessageWs::Ping => {
                // Répond automatiquement au ping
                let pong = MessageWebSocket::nouveau_pong(message.id);
                let etat_lock = etat.lock().await;
                etat_lock.diffuser_message(pong);
            }
            TypeMessageWs::Deconnexion => {
                println!("Déconnexion volontaire de {}", nom_utilisateur);
                return Err("Déconnexion volontaire".into());
            }
            _ => {
                println!("Message non supporté de {}: {:?}", nom_utilisateur, message.type_message);
            }
        }

        Ok(())
    }

    /// Traite les commandes spéciales
    async fn traiter_commande(
        nom_utilisateur: &str,
        etat: &Arc<Mutex<EtatServeurWs>>,
        commande: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let parties: Vec<&str> = commande.split_whitespace().collect();

        match parties.get(0) {
            Some(&"/help") => {
                let aide = "Commandes WebSocket disponibles:\n\
                          /help - Affiche cette aide\n\
                          /users - Liste des utilisateurs connectés\n\
                          /stats - Statistiques du serveur\n\
                          /ping - Teste la connexion\n\
                          /quit - Se déconnecter";
                let reponse = MessageWebSocket::nouvelle_notification(aide.to_string());
                let etat_lock = etat.lock().await;
                etat_lock.diffuser_message(reponse);
            }
            Some(&"/users") => {
                let etat_lock = etat.lock().await;
                let liste = etat_lock.obtenir_liste_utilisateurs();
                let message_liste = format!(
                    "Utilisateurs connectés ({}):\n{}",
                    liste.len(),
                    liste.join("\n• ")
                );
                let reponse = MessageWebSocket::nouvelle_notification(message_liste);
                etat_lock.diffuser_message(reponse);
            }
            Some(&"/stats") => {
                let etat_lock = etat.lock().await;
                let stats = etat_lock.obtenir_statistiques();
                let message_stats = format!("Statistiques du serveur:\n{}", serde_json::to_string_pretty(&stats)?);
                let reponse = MessageWebSocket::nouvelle_notification(message_stats);
                etat_lock.diffuser_message(reponse);
            }
            Some(&"/ping") => {
                let ping = MessageWebSocket::nouveau_ping();
                let etat_lock = etat.lock().await;
                etat_lock.diffuser_message(ping);
            }
            Some(&"/quit") => {
                return Err("Déconnexion demandée".into());
            }
            _ => {
                let reponse = MessageWebSocket::nouvelle_notification(
                    format!("Commande inconnue: {}. Tapez /help pour l'aide.", parties[0])
                );
                let etat_lock = etat.lock().await;
                etat_lock.diffuser_message(reponse);
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Démarrage du serveur WebSocket pour chat en temps réel...");
    
    let serveur = ServeurWebSocket::new("127.0.0.1:9001").await?;
    serveur.demarrer().await?;
    
    Ok(())
}