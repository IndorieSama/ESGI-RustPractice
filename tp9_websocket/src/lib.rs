use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Types de messages WebSocket pour le chat en temps réel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TypeMessageWs {
    /// Message de chat textuel
    Chat,
    /// Message binaire (par exemple, transfert de fichier)
    Binaire,
    /// Connexion d'un nouvel utilisateur
    Connexion,
    /// Déconnexion d'un utilisateur
    Deconnexion,
    /// Notification système
    Notification,
    /// Demande de liste des utilisateurs connectés
    DemandeUtilisateurs,
    /// Réponse avec la liste des utilisateurs
    ListeUtilisateurs,
    /// Ping pour maintenir la connexion
    Ping,
    /// Pong en réponse au ping
    Pong,
}

/// Structure principale des messages WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageWebSocket {
    /// Identifiant unique du message
    pub id: Uuid,
    /// Type de message
    pub type_message: TypeMessageWs,
    /// Nom d'utilisateur de l'expéditeur
    pub utilisateur: Option<String>,
    /// Contenu textuel du message
    pub contenu: Option<String>,
    /// Données binaires (pour les messages binaires)
    pub donnees_binaires: Option<Vec<u8>>,
    /// Métadonnées supplémentaires
    pub metadonnees: Option<serde_json::Value>,
    /// Timestamp du message
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl MessageWebSocket {
    /// Crée un nouveau message de chat
    pub fn nouveau_chat(utilisateur: String, contenu: String) -> Self {
        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::Chat,
            utilisateur: Some(utilisateur),
            contenu: Some(contenu),
            donnees_binaires: None,
            metadonnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un nouveau message binaire
    pub fn nouveau_binaire(utilisateur: String, donnees: Vec<u8>, nom_fichier: Option<String>) -> Self {
        let mut metadonnees = serde_json::Map::new();
        if let Some(nom) = nom_fichier {
            metadonnees.insert("nom_fichier".to_string(), serde_json::Value::String(nom));
        }
        metadonnees.insert("taille".to_string(), serde_json::Value::Number(donnees.len().into()));

        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::Binaire,
            utilisateur: Some(utilisateur),
            contenu: None,
            donnees_binaires: Some(donnees),
            metadonnees: Some(serde_json::Value::Object(metadonnees)),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message de connexion
    pub fn nouvelle_connexion(utilisateur: String) -> Self {
        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::Connexion,
            utilisateur: Some(utilisateur),
            contenu: None,
            donnees_binaires: None,
            metadonnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message de déconnexion
    pub fn nouvelle_deconnexion(utilisateur: String) -> Self {
        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::Deconnexion,
            utilisateur: Some(utilisateur.clone()),
            contenu: Some(format!("{} a quitté le chat", utilisateur)),
            donnees_binaires: None,
            metadonnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une notification système
    pub fn nouvelle_notification(contenu: String) -> Self {
        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::Notification,
            utilisateur: None,
            contenu: Some(contenu),
            donnees_binaires: None,
            metadonnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une demande de liste des utilisateurs
    pub fn nouvelle_demande_utilisateurs() -> Self {
        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::DemandeUtilisateurs,
            utilisateur: None,
            contenu: None,
            donnees_binaires: None,
            metadonnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une réponse avec la liste des utilisateurs
    pub fn nouvelle_liste_utilisateurs(utilisateurs: Vec<String>) -> Self {
        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::ListeUtilisateurs,
            utilisateur: None,
            contenu: None,
            donnees_binaires: None,
            metadonnees: Some(serde_json::to_value(utilisateurs).unwrap()),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message ping
    pub fn nouveau_ping() -> Self {
        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::Ping,
            utilisateur: None,
            contenu: None,
            donnees_binaires: None,
            metadonnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message pong en réponse à un ping
    pub fn nouveau_pong(ping_id: Uuid) -> Self {
        MessageWebSocket {
            id: Uuid::new_v4(),
            type_message: TypeMessageWs::Pong,
            utilisateur: None,
            contenu: None,
            donnees_binaires: None,
            metadonnees: Some(serde_json::to_value(ping_id.to_string()).unwrap()),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Sérialise le message en JSON pour les messages textuels
    pub fn vers_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Désérialise un message depuis JSON
    pub fn depuis_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Détermine si le message doit être envoyé en binaire
    pub fn est_binaire(&self) -> bool {
        matches!(self.type_message, TypeMessageWs::Binaire) || self.donnees_binaires.is_some()
    }

    /// Prépare le message pour l'envoi WebSocket
    pub fn vers_message_websocket(&self) -> Result<tungstenite::Message, Box<dyn std::error::Error + Send + Sync>> {
        if self.est_binaire() {
            // Pour les messages binaires, on sérialise tout en binaire
            let json = self.vers_json()?;
            Ok(tungstenite::Message::Binary(json.into_bytes()))
        } else {
            // Pour les messages textuels, on utilise le format texte
            let json = self.vers_json()?;
            Ok(tungstenite::Message::Text(json))
        }
    }

    /// Parse un message WebSocket entrant
    pub fn depuis_message_websocket(msg: tungstenite::Message) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        match msg {
            tungstenite::Message::Text(texte) => {
                Ok(Self::depuis_json(&texte)?)
            }
            tungstenite::Message::Binary(donnees) => {
                let texte = String::from_utf8(donnees)?;
                Ok(Self::depuis_json(&texte)?)
            }
            _ => Err("Type de message WebSocket non supporté".into())
        }
    }
}

impl fmt::Display for MessageWebSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.type_message {
            TypeMessageWs::Chat => {
                write!(f, "[{}] {}: {}", 
                    self.timestamp.format("%H:%M:%S"),
                    self.utilisateur.as_ref().unwrap_or(&"Anonyme".to_string()),
                    self.contenu.as_ref().unwrap_or(&"".to_string())
                )
            }
            TypeMessageWs::Binaire => {
                let taille = self.donnees_binaires.as_ref().map(|d| d.len()).unwrap_or(0);
                let nom_fichier = self.metadonnees.as_ref()
                    .and_then(|m| m.get("nom_fichier"))
                    .and_then(|n| n.as_str())
                    .unwrap_or("fichier");
                write!(f, "[{}] {} a envoyé un fichier: {} ({} bytes)", 
                    self.timestamp.format("%H:%M:%S"),
                    self.utilisateur.as_ref().unwrap_or(&"Anonyme".to_string()),
                    nom_fichier,
                    taille
                )
            }
            TypeMessageWs::Connexion => {
                write!(f, "[{}] {} s'est connecté", 
                    self.timestamp.format("%H:%M:%S"),
                    self.utilisateur.as_ref().unwrap_or(&"Inconnu".to_string())
                )
            }
            TypeMessageWs::Deconnexion => {
                write!(f, "[{}] {}", 
                    self.timestamp.format("%H:%M:%S"),
                    self.contenu.as_ref().unwrap_or(&"Un utilisateur s'est déconnecté".to_string())
                )
            }
            TypeMessageWs::Notification => {
                write!(f, "[{}] SYSTÈME: {}", 
                    self.timestamp.format("%H:%M:%S"),
                    self.contenu.as_ref().unwrap_or(&"Notification".to_string())
                )
            }
            TypeMessageWs::ListeUtilisateurs => {
                write!(f, "[{}] Utilisateurs connectés", 
                    self.timestamp.format("%H:%M:%S")
                )
            }
            _ => {
                write!(f, "[{}] {:?}", 
                    self.timestamp.format("%H:%M:%S"),
                    self.type_message
                )
            }
        }
    }
}

/// Utilitaires pour la gestion des connexions WebSocket
pub mod utils {
    use super::*;
    
    /// Valide un nom d'utilisateur
    pub fn valider_nom_utilisateur(nom: &str) -> Result<(), String> {
        if nom.is_empty() {
            return Err("Le nom d'utilisateur ne peut pas être vide".to_string());
        }
        
        if nom.len() > 50 {
            return Err("Le nom d'utilisateur ne peut pas dépasser 50 caractères".to_string());
        }
        
        if nom.contains(' ') {
            return Err("Le nom d'utilisateur ne peut pas contenir d'espaces".to_string());
        }
        
        Ok(())
    }
    
    /// Génère un ID de session unique
    pub fn generer_id_session() -> String {
        Uuid::new_v4().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_message_chat() {
        let message = MessageWebSocket::nouveau_chat("Alice".to_string(), "Hello World!".to_string());
        assert_eq!(message.type_message, TypeMessageWs::Chat);
        assert_eq!(message.utilisateur, Some("Alice".to_string()));
        assert_eq!(message.contenu, Some("Hello World!".to_string()));
        assert!(!message.est_binaire());
    }

    #[test]
    fn test_creation_message_binaire() {
        let donnees = vec![1, 2, 3, 4, 5];
        let message = MessageWebSocket::nouveau_binaire("Bob".to_string(), donnees.clone(), Some("test.bin".to_string()));
        assert_eq!(message.type_message, TypeMessageWs::Binaire);
        assert_eq!(message.utilisateur, Some("Bob".to_string()));
        assert_eq!(message.donnees_binaires, Some(donnees));
        assert!(message.est_binaire());
    }

    #[test]
    fn test_serialisation_deserialisation() {
        let message = MessageWebSocket::nouveau_chat("Charlie".to_string(), "Test message".to_string());
        let json = message.vers_json().unwrap();
        let message_deserialise = MessageWebSocket::depuis_json(&json).unwrap();
        
        assert_eq!(message.type_message, message_deserialise.type_message);
        assert_eq!(message.utilisateur, message_deserialise.utilisateur);
        assert_eq!(message.contenu, message_deserialise.contenu);
    }

    #[test]
    fn test_validation_nom_utilisateur() {
        assert!(utils::valider_nom_utilisateur("Alice").is_ok());
        assert!(utils::valider_nom_utilisateur("").is_err());
        assert!(utils::valider_nom_utilisateur("A".repeat(51).as_str()).is_err());
        assert!(utils::valider_nom_utilisateur("Alice Bob").is_err());
    }
}