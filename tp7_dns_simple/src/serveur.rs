use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tp7_dns::{DnsMessage, DnsAnswer, DNS_TYPE_A};

/// Structure représentant le serveur DNS simple
pub struct ServeurDns {
    socket: UdpSocket,
    /// Base de données simple des domaines -> IP
    domaines: HashMap<String, [u8; 4]>,
}

impl ServeurDns {
    /// Crée un nouveau serveur DNS
    pub async fn new(adresse: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(adresse).await?;
        
        // Initialise quelques domaines prédéfinis pour les tests
        let mut domaines = HashMap::new();
        domaines.insert("exemple.com".to_string(), [192, 168, 1, 100]);
        domaines.insert("test.local".to_string(), [127, 0, 0, 1]);
        domaines.insert("serveur.esgi".to_string(), [10, 0, 0, 50]);
        domaines.insert("www.exemple.com".to_string(), [192, 168, 1, 101]);
        
        println!("Serveur DNS démarré sur {}", adresse);
        println!("Domaines configurés :");
        for (domaine, ip) in &domaines {
            println!("   {} -> {}.{}.{}.{}", domaine, ip[0], ip[1], ip[2], ip[3]);
        }
        
        Ok(ServeurDns {
            socket,
            domaines,
        })
    }

    /// Démarre l'écoute des requêtes DNS
    pub async fn demarrer(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = vec![0u8; 512]; // Taille standard DNS
        
        loop {
            // Attend une requête
            match self.socket.recv_from(&mut buffer).await {
                Ok((taille, adresse_client)) => {
                    println!("Requête reçue de {} ({} bytes)", adresse_client, taille);
                    
                    // Traite la requête de manière asynchrone
                    if let Err(e) = self.traiter_requete(&buffer[..taille], adresse_client).await {
                        eprintln!("Erreur lors du traitement de la requête: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Erreur lors de la réception: {}", e);
                }
            }
        }
    }

    /// Traite une requête DNS et envoie la réponse
    async fn traiter_requete(&self, donnees: &[u8], adresse_client: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        // Parse la requête DNS
        let requete = match DnsMessage::from_bytes(donnees) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Erreur parsing requête DNS: {}", e);
                return Ok(());
            }
        };

        println!("ID requête: {}", requete.header.id);
        
        // Crée la réponse
        let mut reponse = DnsMessage::new_response(&requete);
        
        // Traite chaque question
        for question in &requete.questions {
            println!("Question: {} (type: {})", question.qname, question.qtype);
            
            // Vérifie si on connaît ce domaine et si c'est une requête de type A
            if question.qtype == DNS_TYPE_A {
                if let Some(ip) = self.domaines.get(&question.qname) {
                    // Ajoute la réponse
                    let answer = DnsAnswer::new_a_record(
                        question.qname.clone(),
                        *ip,
                        300 // TTL de 5 minutes
                    );
                    
                    reponse.answers.push(answer);
                    reponse.header.ancount += 1;
                    
                    println!("Réponse: {} -> {}.{}.{}.{}", 
                        question.qname, ip[0], ip[1], ip[2], ip[3]);
                } else {
                    println!("Domaine inconnu: {}", question.qname);
                    // Marque comme erreur (NXDOMAIN)
                    reponse.header.flags |= 0x0003; // RCODE = 3 (NXDOMAIN)
                }
            } else {
                println!("Type de requête non supporté: {}", question.qtype);
                // Marque comme non implémenté
                reponse.header.flags |= 0x0004; // RCODE = 4 (Not Implemented)
            }
        }

        // Sérialise et envoie la réponse
        let reponse_bytes = reponse.to_bytes();
        
        match self.socket.send_to(&reponse_bytes, adresse_client).await {
            Ok(bytes_envoyes) => {
                println!("Réponse envoyée à {} ({} bytes)", adresse_client, bytes_envoyes);
            }
            Err(e) => {
                eprintln!("Erreur envoi réponse: {}", e);
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Démarrage du serveur DNS simple...");
    
    // Crée et démarre le serveur sur un port non privilégié
    let serveur = ServeurDns::new("127.0.0.1:8053").await?;
    
    // Démarre l'écoute
    serveur.demarrer().await?;
    
    Ok(())
}