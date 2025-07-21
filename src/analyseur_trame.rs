use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Structure pour représenter une entrée de log analysée
#[derive(Debug)]
struct EntreeLog {
    timestamp: String,
    niveau: String,
    message: String,
    donnees: Option<String>,
}

impl EntreeLog {
    // Parse une ligne de log au format [timestamp] NIVEAU - message | Data: donnees
    fn parse_ligne_log(ligne: &str) -> Option<Self> {
        if ligne.starts_with('[') && ligne.contains(']') {
            let parties: Vec<&str> = ligne.splitn(2, ']').collect();
            if parties.len() == 2 {
                let timestamp = parties[0][1..].to_string(); // Enlever le '['
                let reste = parties[1].trim();
                
                if let Some(pos_tiret) = reste.find(" - ") {
                    let niveau = reste[..pos_tiret].to_string();
                    let message_et_data = &reste[pos_tiret + 3..];
                    
                    // Séparer le message des données
                    if let Some(pos_data) = message_et_data.find(" | Data: ") {
                        let message = message_et_data[..pos_data].to_string();
                        let donnees = Some(message_et_data[pos_data + 8..].to_string());
                        return Some(EntreeLog { timestamp, niveau, message, donnees });
                    } else {
                        let message = message_et_data.to_string();
                        return Some(EntreeLog { timestamp, niveau, message, donnees: None });
                    }
                }
            }
        }
        None
    }
}

// Fonction pour lire et analyser un fichier de log
fn analyser_fichier_log(chemin: &str) -> io::Result<Vec<EntreeLog>> {
    let fichier = File::open(chemin)?;
    let lecteur = BufReader::new(fichier);
    let mut entrees = Vec::new();
    
    for ligne in lecteur.lines() {
        let ligne = ligne?;
        if let Some(entree) = EntreeLog::parse_ligne_log(&ligne) {
            entrees.push(entree);
        }
    }
    
    Ok(entrees)
}

// Fonction pour analyser les statistiques du log
fn analyser_statistiques_log(entrees: &[EntreeLog]) {
    let mut compteurs = std::collections::HashMap::new();
    let mut avec_donnees = 0;
    
    for entree in entrees {
        *compteurs.entry(&entree.niveau).or_insert(0) += 1;
        if entree.donnees.is_some() {
            avec_donnees += 1;
        }
    }
    
    println!("Statistiques du log :");
    println!("- Total d'entrées : {}", entrees.len());
    println!("- Entrées avec données : {}", avec_donnees);
    println!("- Répartition par niveau :");
    
    for (niveau, count) in compteurs {
        println!("  • {} : {} entrées", niveau, count);
    }
}

// Fonction pour filtrer les entrées par niveau
fn filtrer_par_niveau<'a>(entrees: &'a [EntreeLog], niveau_recherche: &str) -> Vec<&'a EntreeLog> {
    entrees.iter()
        .filter(|entree| entree.niveau == niveau_recherche)
        .collect()
}

fn main() {
    println!("=== LECTEUR DE TRAMES STRUCTUREES ===\n");
    
    // Liste des fichiers de log à analyser
    let fichiers_log = [
        "src/session_principale.log",
        "src/securite.log", 
        "src/temps_reel.log"
    ];
    
    for fichier in &fichiers_log {
        if !Path::new(fichier).exists() {
            println!("ATTENTION : Le fichier '{}' n'existe pas, ignoré.", fichier);
            continue;
        }
        
        println!("Analyse du fichier : {}", fichier);
        
        match analyser_fichier_log(fichier) {
            Ok(entrees) => {
                if entrees.is_empty() {
                    println!("  ATTENTION : Aucune entrée de log valide trouvée.");
                    continue;
                }
                
                println!("  OK : {} entrées de log analysées", entrees.len());
                
                // Afficher les première et dernière entrées
                if let Some(premiere) = entrees.first() {
                    println!("  Première entrée : [{}] {} - {}", 
                             premiere.timestamp, premiere.niveau, premiere.message);
                }
                if let Some(derniere) = entrees.last() {
                    println!("  Dernière entrée : [{}] {} - {}", 
                             derniere.timestamp, derniere.niveau, derniere.message);
                }
                
                // Statistiques
                analyser_statistiques_log(&entrees);
                
                // Exemples de filtrage
                let erreurs = filtrer_par_niveau(&entrees, "ERROR");
                if !erreurs.is_empty() {
                    println!("  ERREURS détectées :");
                    for erreur in erreurs {
                        println!("    - {}", erreur.message);
                        if let Some(ref donnees) = erreur.donnees {
                            println!("      Détails: {}", donnees);
                        }
                    }
                }
                
                let warnings = filtrer_par_niveau(&entrees, "WARNING");
                if !warnings.is_empty() {
                    println!("  AVERTISSEMENTS :");
                    for warning in warnings {
                        println!("    - {}", warning.message);
                    }
                }
                
                let transactions = filtrer_par_niveau(&entrees, "TRANSACTION");
                if !transactions.is_empty() {
                    println!("  TRANSACTIONS :");
                    for transaction in transactions {
                        println!("    - {}", transaction.message);
                        if let Some(ref donnees) = transaction.donnees {
                            println!("      Détails: {}", donnees);
                        }
                    }
                }
                
                let securite = filtrer_par_niveau(&entrees, "SECURITY");
                if !securite.is_empty() {
                    println!("  EVENEMENTS DE SECURITE :");
                    for evt in securite {
                        println!("    - {}", evt.message);
                        if let Some(ref donnees) = evt.donnees {
                            println!("      Détails: {}", donnees);
                        }
                    }
                }
            }
            Err(e) => {
                println!("  ERREUR lors de l'analyse : {}", e);
            }
        }
        
        println!(); // Ligne vide entre les fichiers
    }
    
    println!("Analyse terminée !");
}
