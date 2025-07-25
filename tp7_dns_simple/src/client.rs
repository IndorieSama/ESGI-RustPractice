use std::io::{self, Write};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;
use tp7_dns::{DnsMessage, DNS_TYPE_A};

/// Structure représentant le client DNS
pub struct ClientDns {
    socket: UdpSocket,
    serveur_dns: SocketAddr,
}

impl ClientDns {
    /// Crée un nouveau client DNS
    pub async fn new(serveur_dns: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?; // Bind sur un port aléatoire
        let serveur_dns: SocketAddr = serveur_dns.parse()?;
        
        println!("Client DNS connecté au serveur {}", serveur_dns);
        
        Ok(ClientDns {
            socket,
            serveur_dns,
        })
    }

    /// Résout un nom de domaine en adresse IP
    pub async fn resoudre_domaine(&self, domaine: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        println!("\nRésolution de '{}'...", domaine);
        
        // Génère un ID unique pour la requête
        let id_requete = rand::random::<u16>();
        
        // Crée la requête DNS
        let requete = DnsMessage::new_query(id_requete, domaine.to_string(), DNS_TYPE_A);
        let requete_bytes = requete.to_bytes();
        
        println!("Envoi de la requête (ID: {}, {} bytes)", id_requete, requete_bytes.len());
        
        // Envoie la requête
        self.socket.send_to(&requete_bytes, self.serveur_dns).await?;
        
        // Attend la réponse avec timeout
        let mut buffer = vec![0u8; 512];
        
        match timeout(Duration::from_secs(5), self.socket.recv_from(&mut buffer)).await {
            Ok(Ok((taille, adresse))) => {
                println!("Réponse reçue de {} ({} bytes)", adresse, taille);
                
                // Parse la réponse
                match DnsMessage::from_bytes(&buffer[..taille]) {
                    Ok(reponse) => {
                        // Vérifie que c'est bien une réponse à notre requête
                        if reponse.header.id != id_requete {
                            println!("ATTENTION: ID de réponse incorrect (attendu: {}, reçu: {})", 
                                id_requete, reponse.header.id);
                            return Ok(None);
                        }
                        
                        // Vérifie le code de réponse
                        let rcode = reponse.header.flags & 0x000F;
                        match rcode {
                            0 => {
                                // Succès - parse les réponses
                                if reponse.answers.is_empty() {
                                    println!("Aucune réponse trouvée");
                                    Ok(None)
                                } else {
                                    // Prend la première réponse de type A
                                    for answer in &reponse.answers {
                                        if answer.rtype == DNS_TYPE_A && answer.rdata.len() == 4 {
                                            let ip = format!("{}.{}.{}.{}", 
                                                answer.rdata[0], answer.rdata[1], 
                                                answer.rdata[2], answer.rdata[3]);
                                            println!("{} -> {} (TTL: {}s)", domaine, ip, answer.ttl);
                                            return Ok(Some(ip));
                                        }
                                    }
                                    println!("Aucune réponse IPv4 trouvée");
                                    Ok(None)
                                }
                            }
                            3 => {
                                println!("Domaine inexistant (NXDOMAIN)");
                                Ok(None)
                            }
                            4 => {
                                println!("Type de requête non supporté par le serveur");
                                Ok(None)
                            }
                            _ => {
                                println!("Erreur serveur DNS (code: {})", rcode);
                                Ok(None)
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Erreur parsing réponse DNS: {}", e);
                        Ok(None)
                    }
                }
            }
            Ok(Err(e)) => {
                eprintln!("Erreur réception: {}", e);
                Err(Box::new(e))
            }
            Err(_) => {
                println!("Timeout - Pas de réponse du serveur DNS");
                Ok(None)
            }
        }
    }

    /// Lance une session interactive pour résoudre des domaines
    pub async fn session_interactive(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n=== Client DNS Interactif ===");
        println!("Tapez un nom de domaine à résoudre, ou 'quit' pour quitter.");
        println!("Exemples de domaines configurés: exemple.com, test.local, serveur.esgi\n");
        
        loop {
            // Demande le domaine à l'utilisateur
            print("Domaine à résoudre: ")?;
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let domaine = input.trim();
            
            // Vérifie si l'utilisateur veut quitter
            if domaine.is_empty() {
                continue;
            }
            
            if domaine.eq_ignore_ascii_case("quit") || domaine.eq_ignore_ascii_case("exit") {
                println!("Au revoir !");
                break;
            }
            
            // Résout le domaine
            match self.resoudre_domaine(domaine).await {
                Ok(Some(ip)) => {
                    println!("Résolution réussie: {} -> {}", domaine, ip);
                }
                Ok(None) => {
                    println!("Impossible de résoudre '{}'", domaine);
                }
                Err(e) => {
                    eprintln!("Erreur: {}", e);
                }
            }
        }
        
        Ok(())
    }
}

/// Fonction utilitaire pour afficher un prompt
fn print(msg: &str) -> io::Result<()> {
    print!("{}", msg);
    io::stdout().flush()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Démarrage du client DNS...");
    
    // Se connecte au serveur DNS local
    let client = ClientDns::new("127.0.0.1:8053").await?;
    
    // Teste quelques résolutions automatiques
    println!("\n=== Tests automatiques ===");
    
    let domaines_test = vec![
        "exemple.com",
        "test.local", 
        "serveur.esgi",
        "www.exemple.com",
        "inexistant.com", // Ce domaine n'existe pas dans notre serveur
    ];
    
    for domaine in domaines_test {
        let _ = client.resoudre_domaine(domaine).await;
        tokio::time::sleep(Duration::from_millis(500)).await; // Pause entre les requêtes
    }
    
    // Lance la session interactive
    client.session_interactive().await?;
    
    Ok(())
}