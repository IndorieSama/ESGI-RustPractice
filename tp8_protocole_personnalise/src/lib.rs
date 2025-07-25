use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Types d'opérations du protocole de calcul à distance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TypeOperation {
    /// Connexion initiale d'un client
    Connexion,
    /// Confirmation de connexion par le serveur
    ConnexionOk,
    /// Demande de calcul arithmétique
    Calcul,
    /// Réponse avec le résultat du calcul
    ResultatCalcul,
    /// Demande d'information sur le serveur
    InfoServeur,
    /// Réponse avec les informations du serveur
    ReponseInfoServeur,
    /// Demande de statistiques
    Statistiques,
    /// Réponse avec les statistiques
    ReponseStatistiques,
    /// Message d'erreur
    Erreur,
    /// Ping pour tester la connexion
    Ping,
    /// Pong en réponse au ping
    Pong,
    /// Déconnexion volontaire
    Deconnexion,
}

/// Types d'opérations de calcul supportées
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationMath {
    Addition,
    Soustraction,
    Multiplication,
    Division,
    Puissance,
    Racine,
    Factorielle,
    Fibonacci,
}

/// Structure pour une requête de calcul
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequeteCalcul {
    pub operation: OperationMath,
    pub operande1: f64,
    pub operande2: Option<f64>, // Optionnel pour les opérations unaires
}

/// Structure principale des messages du protocole
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageProtocole {
    /// Identifiant unique du message
    pub id: Uuid,
    /// Type d'opération
    pub type_operation: TypeOperation,
    /// Identifiant de session client (optionnel)
    pub session_id: Option<String>,
    /// Requête de calcul (pour les demandes de calcul)
    pub requete_calcul: Option<RequeteCalcul>,
    /// Résultat du calcul (pour les réponses)
    pub resultat: Option<f64>,
    /// Contenu textuel du message
    pub contenu: Option<String>,
    /// Données supplémentaires (pour statistiques, erreurs, etc.)
    pub donnees: Option<serde_json::Value>,
    /// Timestamp du message
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl MessageProtocole {
    /// Crée un nouveau message de connexion
    pub fn nouvelle_connexion(session_id: String) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::Connexion,
            session_id: Some(session_id),
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message de confirmation de connexion
    pub fn nouvelle_connexion_ok(message_bienvenue: String) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::ConnexionOk,
            session_id: None,
            requete_calcul: None,
            resultat: None,
            contenu: Some(message_bienvenue),
            donnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une nouvelle requête de calcul
    pub fn nouvelle_requete_calcul(session_id: String, requete: RequeteCalcul) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::Calcul,
            session_id: Some(session_id),
            requete_calcul: Some(requete),
            resultat: None,
            contenu: None,
            donnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une réponse avec le résultat du calcul
    pub fn nouveau_resultat_calcul(requete_id: Uuid, resultat: f64, details: Option<String>) -> Self {
        MessageProtocole {
            id: requete_id, // Garde le même ID pour la corrélation
            type_operation: TypeOperation::ResultatCalcul,
            session_id: None,
            requete_calcul: None,
            resultat: Some(resultat),
            contenu: details,
            donnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une demande d'informations sur le serveur
    pub fn nouvelle_demande_info_serveur(session_id: String) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::InfoServeur,
            session_id: Some(session_id),
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une réponse avec les informations du serveur
    pub fn nouvelle_reponse_info_serveur(info: serde_json::Value) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::ReponseInfoServeur,
            session_id: None,
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: Some(info),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une demande de statistiques
    pub fn nouvelle_demande_statistiques(session_id: String) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::Statistiques,
            session_id: Some(session_id),
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée une réponse avec les statistiques
    pub fn nouvelle_reponse_statistiques(stats: serde_json::Value) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::ReponseStatistiques,
            session_id: None,
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: Some(stats),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message d'erreur
    pub fn nouvelle_erreur(code_erreur: String, description: String) -> Self {
        let mut erreur_data = serde_json::Map::new();
        erreur_data.insert("code".to_string(), serde_json::Value::String(code_erreur));
        erreur_data.insert("description".to_string(), serde_json::Value::String(description));

        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::Erreur,
            session_id: None,
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: Some(serde_json::Value::Object(erreur_data)),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message ping
    pub fn nouveau_ping() -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::Ping,
            session_id: None,
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message pong en réponse à un ping
    pub fn nouveau_pong(ping_id: Uuid) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::Pong,
            session_id: None,
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: Some(serde_json::to_value(ping_id.to_string()).unwrap()),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Crée un message de déconnexion
    pub fn nouvelle_deconnexion(session_id: String) -> Self {
        MessageProtocole {
            id: Uuid::new_v4(),
            type_operation: TypeOperation::Deconnexion,
            session_id: Some(session_id),
            requete_calcul: None,
            resultat: None,
            contenu: None,
            donnees: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Sérialise le message en JSON
    pub fn vers_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Désérialise un message depuis JSON
    pub fn depuis_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Sérialise le message en bytes avec préfixe de taille
    /// Format: [taille_message: u32][message_json: bytes]
    pub fn vers_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let json = self.vers_json()?;
        let json_bytes = json.as_bytes();
        let taille = json_bytes.len() as u32;
        
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&taille.to_be_bytes());
        bytes.extend_from_slice(json_bytes);
        
        Ok(bytes)
    }

    /// Désérialise un message depuis des bytes
    pub fn depuis_bytes(bytes: &[u8]) -> Result<(Self, usize), Box<dyn std::error::Error + Send + Sync>> {
        if bytes.len() < 4 {
            return Err("Données insuffisantes pour la taille".into());
        }

        // Lit la taille du message
        let taille = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        
        if bytes.len() < 4 + taille {
            return Err("Données insuffisantes pour le message".into());
        }

        // Extrait le JSON
        let json_bytes = &bytes[4..4 + taille];
        let json = std::str::from_utf8(json_bytes)?;
        let message = Self::depuis_json(json)?;
        
        Ok((message, 4 + taille))
    }
}

impl fmt::Display for MessageProtocole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.type_operation {
            TypeOperation::Calcul => {
                if let Some(ref req) = self.requete_calcul {
                    write!(f, "[{}] Calcul: {:?}({}, {:?})", 
                        self.timestamp.format("%H:%M:%S"),
                        req.operation,
                        req.operande1,
                        req.operande2
                    )
                } else {
                    write!(f, "[{}] Calcul invalide", self.timestamp.format("%H:%M:%S"))
                }
            }
            TypeOperation::ResultatCalcul => {
                write!(f, "[{}] Résultat: {}", 
                    self.timestamp.format("%H:%M:%S"),
                    self.resultat.unwrap_or(0.0)
                )
            }
            TypeOperation::Erreur => {
                write!(f, "[{}] ERREUR: {}", 
                    self.timestamp.format("%H:%M:%S"),
                    self.contenu.as_ref().unwrap_or(&"Erreur inconnue".to_string())
                )
            }
            _ => {
                write!(f, "[{}] {:?}", 
                    self.timestamp.format("%H:%M:%S"),
                    self.type_operation
                )
            }
        }
    }
}

/// Codes d'erreur du protocole
pub mod codes_erreurs {
    pub const SESSION_INVALIDE: &str = "INVALID_SESSION";
    pub const OPERATION_INVALIDE: &str = "INVALID_OPERATION";
    pub const PARAMETRES_INVALIDES: &str = "INVALID_PARAMETERS";
    pub const DIVISION_PAR_ZERO: &str = "DIVISION_BY_ZERO";
    pub const OVERFLOW_MATHEMATIQUE: &str = "MATH_OVERFLOW";
    pub const MESSAGE_MALFORMED: &str = "MALFORMED_MESSAGE";
    pub const NON_AUTHENTIFIE: &str = "NOT_AUTHENTICATED";
    pub const SERVEUR_SURCHARGE: &str = "SERVER_OVERLOADED";
}

/// Utilitaires pour les calculs
pub mod calculateur {
    use super::{RequeteCalcul, OperationMath};

    /// Effectue un calcul basé sur la requête
    pub fn calculer(requete: &RequeteCalcul) -> Result<f64, String> {
        match requete.operation {
            OperationMath::Addition => {
                if let Some(op2) = requete.operande2 {
                    Ok(requete.operande1 + op2)
                } else {
                    Err("Opération Addition nécessite 2 opérandes".to_string())
                }
            }
            OperationMath::Soustraction => {
                if let Some(op2) = requete.operande2 {
                    Ok(requete.operande1 - op2)
                } else {
                    Err("Opération Soustraction nécessite 2 opérandes".to_string())
                }
            }
            OperationMath::Multiplication => {
                if let Some(op2) = requete.operande2 {
                    Ok(requete.operande1 * op2)
                } else {
                    Err("Opération Multiplication nécessite 2 opérandes".to_string())
                }
            }
            OperationMath::Division => {
                if let Some(op2) = requete.operande2 {
                    if op2 == 0.0 {
                        Err("Division par zéro".to_string())
                    } else {
                        Ok(requete.operande1 / op2)
                    }
                } else {
                    Err("Opération Division nécessite 2 opérandes".to_string())
                }
            }
            OperationMath::Puissance => {
                if let Some(op2) = requete.operande2 {
                    Ok(requete.operande1.powf(op2))
                } else {
                    Err("Opération Puissance nécessite 2 opérandes".to_string())
                }
            }
            OperationMath::Racine => {
                if requete.operande1 < 0.0 {
                    Err("Racine d'un nombre négatif".to_string())
                } else {
                    Ok(requete.operande1.sqrt())
                }
            }
            OperationMath::Factorielle => {
                if requete.operande1 < 0.0 || requete.operande1.fract() != 0.0 {
                    Err("Factorielle nécessite un entier positif".to_string())
                } else if requete.operande1 > 170.0 {
                    Err("Factorielle trop grande (limite: 170)".to_string())
                } else {
                    let n = requete.operande1 as u64;
                    let mut resultat = 1.0;
                    for i in 1..=n {
                        resultat *= i as f64;
                    }
                    Ok(resultat)
                }
            }
            OperationMath::Fibonacci => {
                if requete.operande1 < 0.0 || requete.operande1.fract() != 0.0 {
                    Err("Fibonacci nécessite un entier positif".to_string())
                } else if requete.operande1 > 78.0 {
                    Err("Fibonacci trop grand (limite: 78)".to_string())
                } else {
                    let n = requete.operande1 as u64;
                    if n <= 1 {
                        Ok(n as f64)
                    } else {
                        let mut a = 0.0;
                        let mut b = 1.0;
                        for _ in 2..=n {
                            let temp = a + b;
                            a = b;
                            b = temp;
                        }
                        Ok(b)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::calculateur::calculer;

    #[test]
    fn test_creation_requete_calcul() {
        let requete = RequeteCalcul {
            operation: OperationMath::Addition,
            operande1: 5.0,
            operande2: Some(3.0),
        };
        let message = MessageProtocole::nouvelle_requete_calcul("test_session".to_string(), requete);
        assert_eq!(message.type_operation, TypeOperation::Calcul);
        assert_eq!(message.session_id, Some("test_session".to_string()));
    }

    #[test]
    fn test_calcul_addition() {
        let requete = RequeteCalcul {
            operation: OperationMath::Addition,
            operande1: 5.0,
            operande2: Some(3.0),
        };
        let resultat = calculer(&requete).unwrap();
        assert_eq!(resultat, 8.0);
    }

    #[test]
    fn test_calcul_division_par_zero() {
        let requete = RequeteCalcul {
            operation: OperationMath::Division,
            operande1: 5.0,
            operande2: Some(0.0),
        };
        let resultat = calculer(&requete);
        assert!(resultat.is_err());
    }

    #[test]
    fn test_serialisation_deserialisation() {
        let requete = RequeteCalcul {
            operation: OperationMath::Multiplication,
            operande1: 4.0,
            operande2: Some(7.0),
        };
        let message = MessageProtocole::nouvelle_requete_calcul("session123".to_string(), requete);
        let json = message.vers_json().unwrap();
        let message_deserialise = MessageProtocole::depuis_json(&json).unwrap();
        
        assert_eq!(message.type_operation, message_deserialise.type_operation);
        assert_eq!(message.session_id, message_deserialise.session_id);
    }
}