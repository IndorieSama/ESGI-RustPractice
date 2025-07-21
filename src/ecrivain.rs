use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

// Structure pour représenter une entrée de données avec horodatage
#[derive(Debug, Clone)]
struct EntreeDonnee {
    timestamp: u64,
    niveau: String,
    message: String,
    donnees: Option<String>,
}

impl EntreeDonnee {
    fn nouvelle(niveau: &str, message: &str, donnees: Option<&str>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            timestamp,
            niveau: niveau.to_string(),
            message: message.to_string(),
            donnees: donnees.map(|d| d.to_string()),
        }
    }
    
    // Format pour fichier .log (format technique)
    fn vers_format_log(&self) -> String {
        match &self.donnees {
            Some(donnees) => format!("[{}] {} - {} | Data: {}\n", 
                                   self.timestamp, self.niveau, self.message, donnees),
            None => format!("[{}] {} - {}\n", 
                           self.timestamp, self.niveau, self.message),
        }
    }
    
    // Format pour fichier .txt (format lisible)
    fn vers_format_txt(&self) -> String {
        let date_lisible = format_timestamp(self.timestamp);
        match &self.donnees {
            Some(donnees) => format!("{} [{}] {} - Données: {}\n", 
                                   date_lisible, self.niveau, self.message, donnees),
            None => format!("{} [{}] {}\n", 
                           date_lisible, self.niveau, self.message),
        }
    }
}

// Fonction utilitaire pour formater le timestamp en date lisible
fn format_timestamp(timestamp: u64) -> String {
    // Conversion basique du timestamp (pour la démo)
    format!("2025-07-21 {}:{}:{}", 
            18 + (timestamp % 6), 
            (timestamp % 60), 
            (timestamp % 60))
}

// Fonction pour écrire une entrée dans les deux formats
fn ecrire_entree_double_format(entree: &EntreeDonnee, chemin_base: &str) -> io::Result<()> {
    let chemin_log = format!("{}.log", chemin_base);
    let chemin_txt = format!("{}.txt", chemin_base);
    
    // Écriture au format .log
    let mut fichier_log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(chemin_log)?;
    fichier_log.write_all(entree.vers_format_log().as_bytes())?;
    
    // Écriture au format .txt
    let mut fichier_txt = OpenOptions::new()
        .create(true)
        .append(true)
        .open(chemin_txt)?;
    fichier_txt.write_all(entree.vers_format_txt().as_bytes())?;
    
    Ok(())
}

// Fonction pour écrire une trame complète de données
fn ecrire_trame_donnees(chemin_base: &str, trame: &[EntreeDonnee]) -> io::Result<()> {
    let chemin_log = format!("{}.log", chemin_base);
    let chemin_txt = format!("{}.txt", chemin_base);
    
    // Réinitialiser les fichiers
    File::create(&chemin_log)?;
    File::create(&chemin_txt)?;
    
    // En-tête pour le fichier .log
    let mut fichier_log = OpenOptions::new()
        .append(true)
        .open(&chemin_log)?;
    writeln!(fichier_log, "=== JOURNAL TECHNIQUE - SESSION {} ===", 
             SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())?;
    
    // En-tête pour le fichier .txt
    let mut fichier_txt = OpenOptions::new()
        .append(true)
        .open(&chemin_txt)?;
    writeln!(fichier_txt, "=== RAPPORT D'ACTIVITÉ - {} ===", 
             format_timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()))?;
    
    // Écrire chaque entrée de la trame
    for entree in trame {
        fichier_log.write_all(entree.vers_format_log().as_bytes())?;
        fichier_txt.write_all(entree.vers_format_txt().as_bytes())?;
    }
    
    // Pied de page
    writeln!(fichier_log, "=== FIN DU JOURNAL ===")?;
    writeln!(fichier_txt, "=== FIN DU RAPPORT ===")?;
    
    Ok(())
}

fn main() {
    println!("=== ÉCRIVAIN DE FICHIER AVEC TRAME STRUCTURÉE ===\n");
    
    // Créer une trame de données d'exemple
    let mut trame_donnees = Vec::new();
    
    // 1. Démarrage de l'application
    trame_donnees.push(EntreeDonnee::nouvelle("INFO", "Démarrage de l'application", None));
    
    // 2. Opérations bancaires simulées
    trame_donnees.push(EntreeDonnee::nouvelle(
        "TRANSACTION", 
        "Création de compte", 
        Some("Compte Courant - Solde initial: 1500.00€")
    ));
    
    trame_donnees.push(EntreeDonnee::nouvelle(
        "TRANSACTION", 
        "Dépôt effectué", 
        Some("Montant: 250.00€ - Nouveau solde: 1750.00€")
    ));
    
    trame_donnees.push(EntreeDonnee::nouvelle(
        "TRANSACTION", 
        "Retrait effectué", 
        Some("Montant: 100.00€ - Nouveau solde: 1650.00€")
    ));
    
    // 3. Opérations système
    trame_donnees.push(EntreeDonnee::nouvelle("DEBUG", "Sauvegarde des données", None));
    
    trame_donnees.push(EntreeDonnee::nouvelle(
        "WARNING", 
        "Tentative de retrait important", 
        Some("Montant demandé: 2000.00€ - Solde insuffisant")
    ));
    
    trame_donnees.push(EntreeDonnee::nouvelle("ERROR", "Échec de validation", Some("Code erreur: ERR_001")));
    
    // 4. Fin de session
    trame_donnees.push(EntreeDonnee::nouvelle("INFO", "Fin de session utilisateur", None));
    
    // Écrire la trame complète dans les deux formats
    println!("1. Écriture de la trame principale...");
    if let Err(e) = ecrire_trame_donnees("src/session_principale", &trame_donnees) {
        println!("Erreur lors de l'écriture de la trame : {}", e);
    } else {
        println!("Trame écrite dans session_principale.log et session_principale.txt");
    }
    
    // Créer une deuxième trame pour les événements de sécurité
    let mut trame_securite = Vec::new();
    trame_securite.push(EntreeDonnee::nouvelle("SECURITY", "Connexion utilisateur", Some("User ID: user123")));
    trame_securite.push(EntreeDonnee::nouvelle("SECURITY", "Authentification réussie", None));
    trame_securite.push(EntreeDonnee::nouvelle("SECURITY", "Accès aux comptes autorisé", None));
    trame_securite.push(EntreeDonnee::nouvelle("SECURITY", "Tentative d'accès non autorisé", Some("IP: 192.168.1.100")));
    trame_securite.push(EntreeDonnee::nouvelle("SECURITY", "Déconnexion utilisateur", None));
    
    println!("\n2. Écriture de la trame sécurité...");
    if let Err(e) = ecrire_trame_donnees("src/securite", &trame_securite) {
        println!("Erreur lors de l'écriture de la trame sécurité : {}", e);
    } else {
        println!("Trame sécurité écrite dans securite.log et securite.txt");
    }
    
    // Démonstration d'écriture en temps réel
    println!("\n3. Écriture en temps réel d'événements...");
    let evenements_temps_reel = vec![
        EntreeDonnee::nouvelle("REALTIME", "Connexion WebSocket établie", None),
        EntreeDonnee::nouvelle("REALTIME", "Réception de données", Some("Taille: 1024 bytes")),
        EntreeDonnee::nouvelle("REALTIME", "Traitement terminé", Some("Durée: 45ms")),
    ];
    
    for evenement in &evenements_temps_reel {
        if let Err(e) = ecrire_entree_double_format(evenement, "src/temps_reel") {
            println!("Erreur : {}", e);
        } else {
            println!("Événement écrit: {}", evenement.message);
        }
    }
    
    // Vérification des fichiers créés
    println!("\nFichiers créés :");
    let fichiers = [
        "src/session_principale.log", "src/session_principale.txt",
        "src/securite.log", "src/securite.txt",
        "src/temps_reel.log", "src/temps_reel.txt"
    ];
    
    for fichier in &fichiers {
        if Path::new(fichier).exists() {
            println!("OK {}", fichier);
        } else {
            println!("ERREUR {} (non trouvé)", fichier);
        }
    }
    
    println!("\nToutes les trames ont été écrites avec succès !");
    println!("Résumé:");
    println!("- Trame principale: {} entrées", trame_donnees.len());
    println!("- Trame sécurité: {} entrées", trame_securite.len());
    println!("- Événements temps réel: {} entrées", evenements_temps_reel.len());
}
