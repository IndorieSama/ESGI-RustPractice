use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};
use tp8_protocole_personnalise::{MessageProtocole, TypeOperation, RequeteCalcul, calculateur, codes_erreurs};

/// Structure représentant une session client
#[derive(Debug, Clone)]
struct SessionClient {
    session_id: String,
    adresse: SocketAddr,
    connexions_actives: u32,
    calculs_effectues: u32,
    temps_connexion: chrono::DateTime<chrono::Utc>,
}

/// État partagé du serveur de calcul
#[derive(Debug)]
struct EtatServeurCalcul {
    /// Sessions actives des clients
    sessions: HashMap<String, SessionClient>,
    /// Statistiques globales du serveur
    total_connexions: u64,
    total_calculs: u64,
    temps_demarrage: chrono::DateTime<chrono::Utc>,
}

impl EtatServeurCalcul {
    fn new() -> Self {
        EtatServeurCalcul {
            sessions: HashMap::new(),
            total_connexions: 0,
            total_calculs: 0,
            temps_demarrage: chrono::Utc::now(),
        }
    }

    /// Ajoute une nouvelle session client
    fn ajouter_session(&mut self, session_id: String, adresse: SocketAddr) -> Result<(), String> {
        if self.sessions.contains_key(&session_id) {
            return Err(format!("Session '{}' déjà active", session_id));
        }

        let session = SessionClient {
            session_id: session_id.clone(),
            adresse,
            connexions_actives: 1,
            calculs_effectues: 0,
            temps_connexion: chrono::Utc::now(),
        };

        self.sessions.insert(session_id.clone(), session);
        self.total_connexions += 1;

        println!("Nouvelle session '{}' créée pour {}", session_id, adresse);
        Ok(())
    }

    /// Supprime une session client
    fn supprimer_session(&mut self, session_id: &str) {
        if let Some(session) = self.sessions.remove(session_id) {
            println!("Session '{}' fermée (calculs effectués: {})", 
                session_id, session.calculs_effectues);
        }
    }

    /// Incrémente le compteur de calculs pour une session
    fn incrementer_calculs(&mut self, session_id: &str) {
        if let Some(session) = self.sessions.get_mut(session_id) {
            session.calculs_effectues += 1;
            self.total_calculs += 1;
        }
    }

    /// Obtient les informations du serveur
    fn obtenir_info_serveur(&self) -> serde_json::Value {
        serde_json::json!({
            "nom": "Serveur de Calcul à Distance",
            "version": "1.0.0",
            "temps_demarrage": self.temps_demarrage,
            "temps_fonctionnement_secondes": (chrono::Utc::now() - self.temps_demarrage).num_seconds(),
            "sessions_actives": self.sessions.len(),
            "operations_supportees": [
                "Addition", "Soustraction", "Multiplication", "Division",
                "Puissance", "Racine", "Factorielle", "Fibonacci"
            ],
            "protocole": "TCP avec messages JSON",
            "format_message": "Prefixe de taille (4 bytes) + JSON"
        })
    }

    /// Obtient les statistiques du serveur
    fn obtenir_statistiques(&self) -> serde_json::Value {
        let sessions_info: Vec<serde_json::Value> = self.sessions.values().map(|s| {
            serde_json::json!({
                "session_id": s.session_id,
                "adresse": s.adresse.to_string(),
                "calculs_effectues": s.calculs_effectues,
                "temps_connexion": s.temps_connexion,
                "duree_connexion_secondes": (chrono::Utc::now() - s.temps_connexion).num_seconds()
            })
        }).collect();

        serde_json::json!({
            "total_connexions": self.total_connexions,
            "total_calculs": self.total_calculs,
            "sessions_actives": self.sessions.len(),
            "sessions": sessions_info,
            "moyenne_calculs_par_session": if self.total_connexions > 0 { 
                self.total_calculs as f64 / self.total_connexions as f64 
            } else { 
                0.0 
            },
            "temps_demarrage": self.temps_demarrage,
            "temps_fonctionnement": format!("{}s", (chrono::Utc::now() - self.temps_demarrage).num_seconds())
        })
    }
}

/// Serveur de calcul à distance
pub struct ServeurCalcul {
    listener: TcpListener,
    etat: Arc<Mutex<EtatServeurCalcul>>,
}

impl ServeurCalcul {
    /// Crée un nouveau serveur de calcul
    pub async fn new(adresse: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let listener = TcpListener::bind(adresse).await?;
        let etat = Arc::new(Mutex::new(EtatServeurCalcul::new()));

        println!("Serveur de calcul à distance démarré sur {}", adresse);
        println!("Opérations supportées: Addition, Soustraction, Multiplication, Division");
        println!("                      Puissance, Racine, Factorielle, Fibonacci");
        println!("En attente de connexions clients...");

        Ok(ServeurCalcul { listener, etat })
    }

    /// Démarre le serveur et écoute les connexions
    pub async fn demarrer(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        loop {
            match self.listener.accept().await {
                Ok((socket, adresse)) => {
                    println!("Nouvelle connexion TCP depuis {}", adresse);
                    let etat = Arc::clone(&self.etat);

                    // Traite chaque client dans une tâche séparée
                    tokio::spawn(async move {
                        if let Err(e) = Self::gerer_client(socket, adresse, etat).await {
                            eprintln!("Erreur avec le client {}: {}", adresse, e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Erreur d'acceptation de connexion: {}", e);
                }
            }
        }
    }

    /// Gère un client connecté
    async fn gerer_client(
        mut socket: TcpStream,
        adresse: SocketAddr,
        etat: Arc<Mutex<EtatServeurCalcul>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut session_id: Option<String> = None;
        let mut buffer = vec![0u8; 1024];
        let mut donnees_incompletes = Vec::new();

        // Attend le message de connexion initial
        let timeout_connexion = timeout(Duration::from_secs(30), socket.read(&mut buffer)).await;

        match timeout_connexion {
            Ok(Ok(taille)) if taille > 0 => {
                donnees_incompletes.extend_from_slice(&buffer[..taille]);

                // Traite le message de connexion
                match MessageProtocole::depuis_bytes(&donnees_incompletes) {
                    Ok((message, bytes_consommes)) => {
                        donnees_incompletes.drain(..bytes_consommes);

                        if message.type_operation == TypeOperation::Connexion {
                            if let Some(id) = message.session_id {
                                // Ajoute la session
                                let mut etat_lock = etat.lock().await;
                                match etat_lock.ajouter_session(id.clone(), adresse) {
                                    Ok(()) => {
                                        session_id = Some(id.clone());

                                        // Envoie la confirmation de connexion
                                        let confirmation = MessageProtocole::nouvelle_connexion_ok(
                                            format!("Connexion réussie! Session: {}", id)
                                        );
                                        Self::envoyer_message(&mut socket, confirmation).await?;

                                        drop(etat_lock); // Libère le verrou

                                        // Continue le traitement des messages du client
                                        Self::traiter_messages_client(&mut socket, &id, &etat, &mut donnees_incompletes).await?;
                                    }
                                    Err(erreur_msg) => {
                                        let erreur = MessageProtocole::nouvelle_erreur(
                                            codes_erreurs::SESSION_INVALIDE.to_string(),
                                            erreur_msg,
                                        );
                                        Self::envoyer_message(&mut socket, erreur).await?;
                                    }
                                }
                            } else {
                                let erreur = MessageProtocole::nouvelle_erreur(
                                    codes_erreurs::SESSION_INVALIDE.to_string(),
                                    "ID de session requis pour la connexion".to_string(),
                                );
                                Self::envoyer_message(&mut socket, erreur).await?;
                            }
                        } else {
                            let erreur = MessageProtocole::nouvelle_erreur(
                                codes_erreurs::NON_AUTHENTIFIE.to_string(),
                                "Connexion requise avant l'envoi de requêtes".to_string(),
                            );
                            Self::envoyer_message(&mut socket, erreur).await?;
                        }
                    }
                    Err(_) => {
                        let erreur = MessageProtocole::nouvelle_erreur(
                            codes_erreurs::MESSAGE_MALFORMED.to_string(),
                            "Message mal formé".to_string(),
                        );
                        Self::envoyer_message(&mut socket, erreur).await?;
                    }
                }
            }
            _ => {
                println!("Timeout ou erreur de connexion pour {}", adresse);
            }
        }

        // Nettoyage: supprime la session si elle était créée
        if let Some(id) = session_id {
            let mut etat_lock = etat.lock().await;
            etat_lock.supprimer_session(&id);
        }

        Ok(())
    }

    /// Traite les messages continus d'un client authentifié
    async fn traiter_messages_client(
        socket: &mut TcpStream,
        session_id: &str,
        etat: &Arc<Mutex<EtatServeurCalcul>>,
        donnees_incompletes: &mut Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut buffer = vec![0u8; 1024];

        loop {
            // Essaie de lire plus de données
            match timeout(Duration::from_secs(300), socket.read(&mut buffer)).await {
                Ok(Ok(0)) => {
                    // Connexion fermée par le client
                    println!("Client {} (session: {}) a fermé la connexion", socket.peer_addr()?, session_id);
                    break;
                }
                Ok(Ok(taille)) => {
                    donnees_incompletes.extend_from_slice(&buffer[..taille]);

                    // Traite tous les messages complets dans le buffer
                    while donnees_incompletes.len() >= 4 {
                        match MessageProtocole::depuis_bytes(donnees_incompletes) {
                            Ok((message, bytes_consommes)) => {
                                donnees_incompletes.drain(..bytes_consommes);
                                Self::traiter_message_client(socket, session_id, etat, message).await?;
                            }
                            Err(_) => {
                                // Pas assez de données pour un message complet
                                break;
                            }
                        }
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Erreur de lecture pour session {}: {}", session_id, e);
                    break;
                }
                Err(_) => {
                    // Timeout de 5 minutes - envoie un ping
                    let ping = MessageProtocole::nouveau_ping();
                    if Self::envoyer_message(socket, ping).await.is_err() {
                        println!("Session {} ne répond plus", session_id);
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Traite un message spécifique d'un client
    async fn traiter_message_client(
        socket: &mut TcpStream,
        session_id: &str,
        etat: &Arc<Mutex<EtatServeurCalcul>>,
        message: MessageProtocole,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match message.type_operation {
            TypeOperation::Calcul => {
                if let Some(requete) = message.requete_calcul {
                    println!("Calcul demandé par {}: {:?}({}, {:?})", 
                        session_id, requete.operation, requete.operande1, requete.operande2);

                    // Effectue le calcul
                    match calculateur::calculer(&requete) {
                        Ok(resultat) => {
                            // Incrémente le compteur de calculs
                            let mut etat_lock = etat.lock().await;
                            etat_lock.incrementer_calculs(session_id);
                            drop(etat_lock);

                            // Envoie le résultat
                            let reponse = MessageProtocole::nouveau_resultat_calcul(
                                message.id,
                                resultat,
                                Some(format!("{:?}({}, {:?}) = {}", 
                                    requete.operation, requete.operande1, requete.operande2, resultat))
                            );
                            Self::envoyer_message(socket, reponse).await?;
                            
                            println!("Résultat envoyé à {}: {}", session_id, resultat);
                        }
                        Err(erreur_calcul) => {
                            let erreur = MessageProtocole::nouvelle_erreur(
                                codes_erreurs::PARAMETRES_INVALIDES.to_string(),
                                erreur_calcul,
                            );
                            Self::envoyer_message(socket, erreur).await?;
                        }
                    }
                } else {
                    let erreur = MessageProtocole::nouvelle_erreur(
                        codes_erreurs::PARAMETRES_INVALIDES.to_string(),
                        "Requête de calcul invalide".to_string(),
                    );
                    Self::envoyer_message(socket, erreur).await?;
                }
            }
            TypeOperation::InfoServeur => {
                let etat_lock = etat.lock().await;
                let info = etat_lock.obtenir_info_serveur();
                drop(etat_lock);
                
                let reponse = MessageProtocole::nouvelle_reponse_info_serveur(info);
                Self::envoyer_message(socket, reponse).await?;
            }
            TypeOperation::Statistiques => {
                let etat_lock = etat.lock().await;
                let stats = etat_lock.obtenir_statistiques();
                drop(etat_lock);
                
                let reponse = MessageProtocole::nouvelle_reponse_statistiques(stats);
                Self::envoyer_message(socket, reponse).await?;
            }
            TypeOperation::Ping => {
                // Répond automatiquement au ping
                let pong = MessageProtocole::nouveau_pong(message.id);
                Self::envoyer_message(socket, pong).await?;
            }
            TypeOperation::Deconnexion => {
                println!("Déconnexion volontaire de la session {}", session_id);
                return Err("Déconnexion volontaire".into());
            }
            _ => {
                let erreur = MessageProtocole::nouvelle_erreur(
                    codes_erreurs::OPERATION_INVALIDE.to_string(),
                    format!("Opération non supportée: {:?}", message.type_operation),
                );
                Self::envoyer_message(socket, erreur).await?;
            }
        }

        Ok(())
    }

    /// Envoie un message à un client
    async fn envoyer_message(
        socket: &mut TcpStream,
        message: MessageProtocole,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bytes = message.vers_bytes()?;
        socket.write_all(&bytes).await?;
        socket.flush().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Démarrage du serveur de calcul à distance...");
    
    let serveur = ServeurCalcul::new("127.0.0.1:8081").await?;
    serveur.demarrer().await?;
    
    Ok(())
}