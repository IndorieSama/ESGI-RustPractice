use std::io::{self, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tp8_protocole_personnalise::{MessageProtocole, TypeOperation, RequeteCalcul, OperationMath};

/// Client pour le protocole de calcul à distance
pub struct ClientCalcul {
    socket: TcpStream,
    session_id: String,
    connecte: bool,
}

impl ClientCalcul {
    /// Se connecte au serveur de calcul
    pub async fn connecter(
        adresse_serveur: &str,
        session_id: String,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        println!("Connexion au serveur de calcul {}...", adresse_serveur);
        
        let mut socket = TcpStream::connect(adresse_serveur).await?;
        
        // Envoie le message de connexion
        let message_connexion = MessageProtocole::nouvelle_connexion(session_id.clone());
        let bytes = message_connexion.vers_bytes()?;
        socket.write_all(&bytes).await?;
        socket.flush().await?;
        
        println!("Message de connexion envoyé, en attente de confirmation...");
        
        // Attend la confirmation
        let mut buffer = vec![0u8; 1024];
        match timeout(Duration::from_secs(10), socket.read(&mut buffer)).await {
            Ok(Ok(taille)) if taille > 0 => {
                match MessageProtocole::depuis_bytes(&buffer[..taille]) {
                    Ok((message, _)) => {
                        match message.type_operation {
                            TypeOperation::ConnexionOk => {
                                println!("Connexion réussie!");
                                if let Some(bienvenue) = message.contenu {
                                    println!("{}", bienvenue);
                                }
                                
                                Ok(ClientCalcul {
                                    socket,
                                    session_id,
                                    connecte: true,
                                })
                            }
                            TypeOperation::Erreur => {
                                let erreur_msg = message.contenu.unwrap_or("Erreur inconnue".to_string());
                                return Err(format!("Erreur de connexion: {}", erreur_msg).into());
                            }
                            _ => {
                                return Err("Réponse inattendue du serveur".into());
                            }
                        }
                    }
                    Err(e) => {
                        return Err(format!("Erreur de parsing: {}", e).into());
                    }
                }
            }
            _ => {
                return Err("Timeout lors de la connexion".into());
            }
        }
    }

    /// Lance la session de calcul interactive
    pub async fn demarrer_session(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.connecte {
            return Err("Non connecté au serveur".into());
        }

        println!("\n=== Session de Calcul Démarrée ===");
        println!("Serveur de calcul à distance connecté!");
        println!("Opérations disponibles:");
        println!("  • addition <a> <b>      - Addition de deux nombres");
        println!("  • soustraction <a> <b>  - Soustraction de deux nombres");
        println!("  • multiplication <a> <b> - Multiplication de deux nombres");
        println!("  • division <a> <b>      - Division de deux nombres");
        println!("  • puissance <a> <b>     - Puissance (a^b)");
        println!("  • racine <a>            - Racine carrée");
        println!("  • factorielle <n>       - Factorielle d'un entier");
        println!("  • fibonacci <n>         - Nième nombre de Fibonacci");
        println!("  • info                  - Informations du serveur");
        println!("  • stats                 - Statistiques du serveur");
        println!("  • ping                  - Test de connexion");
        println!("  • quit                  - Quitter");
        println!("=====================================\n");

        self.boucle_principale().await?;
        
        Ok(())
    }

    /// Boucle principale du client
    async fn boucle_principale(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut donnees_incompletes = Vec::new();
        let mut buffer = vec![0u8; 1024];

        loop {
            // Affiche le prompt et lit l'input utilisateur
            print!("calcul> ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            // Traite la commande /quit localement
            if input.eq_ignore_ascii_case("quit") {
                println!("Déconnexion...");
                let deconnexion = MessageProtocole::nouvelle_deconnexion(self.session_id.clone());
                let _ = self.envoyer_message(deconnexion).await;
                break;
            }
            
            // Parse et traite la commande
            if let Some(message) = self.parser_commande(input).await? {
                // Envoie la requête au serveur
                if self.envoyer_message(message).await.is_err() {
                    eprintln!("Erreur d'envoi");
                    break;
                }
                
                // Attend la réponse du serveur
                match timeout(Duration::from_secs(10), self.socket.read(&mut buffer)).await {
                    Ok(Ok(0)) => {
                        println!("Connexion fermée par le serveur");
                        break;
                    }
                    Ok(Ok(taille)) => {
                        donnees_incompletes.extend_from_slice(&buffer[..taille]);
                        
                        // Traite la réponse
                        if donnees_incompletes.len() >= 4 {
                            match MessageProtocole::depuis_bytes(&donnees_incompletes) {
                                Ok((message, bytes_consommes)) => {
                                    donnees_incompletes.drain(..bytes_consommes);
                                    self.traiter_reponse(message).await;
                                }
                                Err(e) => {
                                    eprintln!("Erreur de parsing de la réponse: {}", e);
                                }
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        eprintln!("Erreur de lecture: {}", e);
                        break;
                    }
                    Err(_) => {
                        println!("Timeout - aucune réponse du serveur");
                    }
                }
            }
        }
        
        self.connecte = false;
        Ok(())
    }

    /// Parse une commande utilisateur et crée le message approprié
    async fn parser_commande(&self, input: &str) -> Result<Option<MessageProtocole>, Box<dyn std::error::Error + Send + Sync>> {
        let parties: Vec<&str> = input.split_whitespace().collect();
        if parties.is_empty() {
            return Ok(None);
        }
        
        match parties[0].to_lowercase().as_str() {
            "addition" | "add" => {
                if parties.len() != 3 {
                    println!("Usage: addition <nombre1> <nombre2>");
                    return Ok(None);
                }
                let a: f64 = parties[1].parse()?;
                let b: f64 = parties[2].parse()?;
                let requete = RequeteCalcul {
                    operation: OperationMath::Addition,
                    operande1: a,
                    operande2: Some(b),
                };
                Ok(Some(MessageProtocole::nouvelle_requete_calcul(self.session_id.clone(), requete)))
            }
            "soustraction" | "sub" => {
                if parties.len() != 3 {
                    println!("Usage: soustraction <nombre1> <nombre2>");
                    return Ok(None);
                }
                let a: f64 = parties[1].parse()?;
                let b: f64 = parties[2].parse()?;
                let requete = RequeteCalcul {
                    operation: OperationMath::Soustraction,
                    operande1: a,
                    operande2: Some(b),
                };
                Ok(Some(MessageProtocole::nouvelle_requete_calcul(self.session_id.clone(), requete)))
            }
            "multiplication" | "mul" => {
                if parties.len() != 3 {
                    println!("Usage: multiplication <nombre1> <nombre2>");
                    return Ok(None);
                }
                let a: f64 = parties[1].parse()?;
                let b: f64 = parties[2].parse()?;
                let requete = RequeteCalcul {
                    operation: OperationMath::Multiplication,
                    operande1: a,
                    operande2: Some(b),
                };
                Ok(Some(MessageProtocole::nouvelle_requete_calcul(self.session_id.clone(), requete)))
            }
            "division" | "div" => {
                if parties.len() != 3 {
                    println!("Usage: division <nombre1> <nombre2>");
                    return Ok(None);
                }
                let a: f64 = parties[1].parse()?;
                let b: f64 = parties[2].parse()?;
                let requete = RequeteCalcul {
                    operation: OperationMath::Division,
                    operande1: a,
                    operande2: Some(b),
                };
                Ok(Some(MessageProtocole::nouvelle_requete_calcul(self.session_id.clone(), requete)))
            }
            "puissance" | "pow" => {
                if parties.len() != 3 {
                    println!("Usage: puissance <base> <exposant>");
                    return Ok(None);
                }
                let a: f64 = parties[1].parse()?;
                let b: f64 = parties[2].parse()?;
                let requete = RequeteCalcul {
                    operation: OperationMath::Puissance,
                    operande1: a,
                    operande2: Some(b),
                };
                Ok(Some(MessageProtocole::nouvelle_requete_calcul(self.session_id.clone(), requete)))
            }
            "racine" | "sqrt" => {
                if parties.len() != 2 {
                    println!("Usage: racine <nombre>");
                    return Ok(None);
                }
                let a: f64 = parties[1].parse()?;
                let requete = RequeteCalcul {
                    operation: OperationMath::Racine,
                    operande1: a,
                    operande2: None,
                };
                Ok(Some(MessageProtocole::nouvelle_requete_calcul(self.session_id.clone(), requete)))
            }
            "factorielle" | "fact" => {
                if parties.len() != 2 {
                    println!("Usage: factorielle <entier>");
                    return Ok(None);
                }
                let a: f64 = parties[1].parse()?;
                let requete = RequeteCalcul {
                    operation: OperationMath::Factorielle,
                    operande1: a,
                    operande2: None,
                };
                Ok(Some(MessageProtocole::nouvelle_requete_calcul(self.session_id.clone(), requete)))
            }
            "fibonacci" | "fib" => {
                if parties.len() != 2 {
                    println!("Usage: fibonacci <position>");
                    return Ok(None);
                }
                let a: f64 = parties[1].parse()?;
                let requete = RequeteCalcul {
                    operation: OperationMath::Fibonacci,
                    operande1: a,
                    operande2: None,
                };
                Ok(Some(MessageProtocole::nouvelle_requete_calcul(self.session_id.clone(), requete)))
            }
            "info" => {
                Ok(Some(MessageProtocole::nouvelle_demande_info_serveur(self.session_id.clone())))
            }
            "stats" => {
                Ok(Some(MessageProtocole::nouvelle_demande_statistiques(self.session_id.clone())))
            }
            "ping" => {
                Ok(Some(MessageProtocole::nouveau_ping()))
            }
            _ => {
                println!("Commande inconnue: {}. Tapez 'quit' pour quitter.", parties[0]);
                Ok(None)
            }
        }
    }

    /// Traite une réponse du serveur
    async fn traiter_reponse(&self, message: MessageProtocole) {
        match message.type_operation {
            TypeOperation::ResultatCalcul => {
                if let Some(resultat) = message.resultat {
                    println!("Résultat: {}", resultat);
                    if let Some(details) = message.contenu {
                        println!("Détails: {}", details);
                    }
                } else {
                    println!("Résultat de calcul invalide");
                }
            }
            TypeOperation::ReponseInfoServeur => {
                if let Some(info) = message.donnees {
                    println!("\n=== Informations du Serveur ===");
                    if let Ok(info_obj) = serde_json::from_value::<serde_json::Value>(info) {
                        Self::afficher_json_formate(&info_obj, 0);
                    }
                    println!("==============================\n");
                }
            }
            TypeOperation::ReponseStatistiques => {
                if let Some(stats) = message.donnees {
                    println!("\n=== Statistiques du Serveur ===");
                    if let Ok(stats_obj) = serde_json::from_value::<serde_json::Value>(stats) {
                        Self::afficher_json_formate(&stats_obj, 0);
                    }
                    println!("==============================\n");
                }
            }
            TypeOperation::Pong => {
                println!("Pong reçu - Connexion active");
            }
            TypeOperation::Erreur => {
                if let Some(donnees) = message.donnees {
                    if let (Some(code), Some(description)) = (
                        donnees.get("code").and_then(|c| c.as_str()),
                        donnees.get("description").and_then(|d| d.as_str())
                    ) {
                        println!("ERREUR [{}]: {}", code, description);
                    } else {
                        println!("ERREUR: {}", message.contenu.unwrap_or("Erreur inconnue".to_string()));
                    }
                } else {
                    println!("ERREUR: {}", message.contenu.unwrap_or("Erreur inconnue".to_string()));
                }
            }
            _ => {
                println!("Réponse non gérée: {:?}", message.type_operation);
            }
        }
    }

    /// Affiche un JSON de manière formatée
    fn afficher_json_formate(valeur: &serde_json::Value, indentation: usize) {
        let indent = "  ".repeat(indentation);
        
        match valeur {
            serde_json::Value::Object(map) => {
                for (cle, val) in map {
                    match val {
                        serde_json::Value::Object(_) | serde_json::Value::Array(_) => {
                            println!("{}{}:", indent, cle);
                            Self::afficher_json_formate(val, indentation + 1);
                        }
                        _ => {
                            println!("{}{}: {}", indent, cle, Self::formater_valeur_json(val));
                        }
                    }
                }
            }
            serde_json::Value::Array(arr) => {
                for (index, val) in arr.iter().enumerate() {
                    println!("{}[{}] {}", indent, index, Self::formater_valeur_json(val));
                }
            }
            _ => {
                println!("{}{}", indent, Self::formater_valeur_json(valeur));
            }
        }
    }

    /// Formate une valeur JSON pour l'affichage
    fn formater_valeur_json(valeur: &serde_json::Value) -> String {
        match valeur {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "null".to_string(),
            _ => valeur.to_string(),
        }
    }

    /// Envoie un message au serveur
    async fn envoyer_message(&mut self, message: MessageProtocole) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bytes = message.vers_bytes()?;
        self.socket.write_all(&bytes).await?;
        self.socket.flush().await?;
        Ok(())
    }

    /// Demande l'ID de session à l'utilisateur
    fn demander_session_id() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        loop {
            print!("Entrez votre ID de session (ou appuyez sur Entrée pour un ID automatique): ");
            io::stdout().flush()?;
            
            let mut session_id = String::new();
            io::stdin().read_line(&mut session_id)?;
            let session_id = session_id.trim().to_string();
            
            if session_id.is_empty() {
                // Génère un ID automatique
                return Ok(format!("client_{}", uuid::Uuid::new_v4().to_string()[..8].to_string()));
            }
            
            if session_id.len() > 50 {
                println!("L'ID de session ne peut pas dépasser 50 caractères.");
                continue;
            }
            
            return Ok(session_id);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== Client de Calcul à Distance ===");
    
    // Demande l'ID de session
    let session_id = ClientCalcul::demander_session_id()?;
    
    // Se connecte au serveur
    match ClientCalcul::connecter("127.0.0.1:8081", session_id).await {
        Ok(mut client) => {
            // Démarre la session de calcul
            if let Err(e) = client.demarrer_session().await {
                eprintln!("Erreur durant la session: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Impossible de se connecter: {}", e);
            std::process::exit(1);
        }
    }
    
    println!("Session terminée. Au revoir!");
    Ok(())
}