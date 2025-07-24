use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use chrono::{DateTime, Local};

#[derive(Debug)]
pub struct GestionnaireFichiers {
    pub chemin_base: String,
}

impl GestionnaireFichiers {
    pub fn nouveau(chemin_base: String) -> Self {
        Self { chemin_base }
    }

    pub fn lire_fichier(&self, nom_fichier: &str) -> Result<String, io::Error> {
        let chemin_complet = format!("{}/{}", self.chemin_base, nom_fichier);
        
        match fs::read_to_string(&chemin_complet) {
            Ok(contenu) => {
                println!("Lecture réussie de: {}", nom_fichier);
                self.afficher_info_fichier(&chemin_complet);
                Ok(contenu)
            }
            Err(e) => {
                println!(" Erreur lors de la lecture de {}: {}", nom_fichier, e);
                Err(e)
            }
        }
    }

    pub fn ecrire_fichier(&self, nom_fichier: &str, contenu: &str) -> Result<(), io::Error> {
        let chemin_complet = format!("{}/{}", self.chemin_base, nom_fichier);
        
        match File::create(&chemin_complet) {
            Ok(mut fichier) => {
                match fichier.write_all(contenu.as_bytes()) {
                    Ok(_) => {
                        println!(" Écriture réussie dans: {}", nom_fichier);
                        self.afficher_info_fichier(&chemin_complet);
                        Ok(())
                    }
                    Err(e) => {
                        println!(" Erreur lors de l'écriture dans {}: {}", nom_fichier, e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                println!(" Impossible de créer {}: {}", nom_fichier, e);
                Err(e)
            }
        }
    }

    pub fn modifier_fichier(&self, nom_fichier: &str, nouveau_contenu: &str) -> Result<(), io::Error> {
        let chemin_complet = format!("{}/{}", self.chemin_base, nom_fichier);
        
        if !Path::new(&chemin_complet).exists() {
            println!(" Le fichier {} n'existe pas", nom_fichier);
            return Err(io::Error::new(io::ErrorKind::NotFound, "Fichier non trouvé"));
        }

        match OpenOptions::new().write(true).truncate(true).open(&chemin_complet) {
            Ok(mut fichier) => {
                match fichier.write_all(nouveau_contenu.as_bytes()) {
                    Ok(_) => {
                        println!(" Modification réussie de: {}", nom_fichier);
                        self.afficher_info_fichier(&chemin_complet);
                        Ok(())
                    }
                    Err(e) => {
                        println!(" Erreur lors de la modification de {}: {}", nom_fichier, e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                println!(" Impossible d'ouvrir {} pour modification: {}", nom_fichier, e);
                Err(e)
            }
        }
    }

    pub fn supprimer_fichier(&self, nom_fichier: &str) -> Result<(), io::Error> {
        let chemin_complet = format!("{}/{}", self.chemin_base, nom_fichier);
        
        if !Path::new(&chemin_complet).exists() {
            println!(" Le fichier {} n'existe pas", nom_fichier);
            return Err(io::Error::new(io::ErrorKind::NotFound, "Fichier non trouvé"));
        }

        match fs::remove_file(&chemin_complet) {
            Ok(_) => {
                println!(" Suppression définitive réussie de: {}", nom_fichier);
                Ok(())
            }
            Err(e) => {
                println!(" Erreur lors de la suppression de {}: {}", nom_fichier, e);
                Err(e)
            }
        }
    }

    pub fn lister_fichiers(&self) -> Result<Vec<String>, io::Error> {
        match fs::read_dir(&self.chemin_base) {
            Ok(entries) => {
                let mut fichiers = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(nom) = entry.file_name().to_str() {
                            fichiers.push(nom.to_string());
                        }
                    }
                }
                Ok(fichiers)
            }
            Err(e) => {
                println!(" Erreur lors de la lecture du répertoire: {}", e);
                Err(e)
            }
        }
    }

    fn afficher_info_fichier(&self, chemin: &str) {
        if let Ok(metadata) = fs::metadata(chemin) {
            let taille = metadata.len();
            if let Ok(modified) = metadata.modified() {
                let datetime: DateTime<Local> = modified.into();
                println!(" Taille: {} octets | Modifié: {}", taille, datetime.format("%Y-%m-%d %H:%M:%S"));
            }
        }
    }

    pub fn menu_interactif(&self) {
        loop {
            println!("\n=== GESTIONNAIRE DE FICHIERS ===");
            println!(" Répertoire actuel: {}", self.chemin_base);
            println!("1. Lire un fichier");
            println!("2. Écrire dans un fichier");
            println!("3. Modifier un fichier");
            println!("4. Supprimer un fichier");
            println!("5. Lister les fichiers");
            println!("6. Quitter");
            println!("================================");
            print!("Choisissez une option (1-6): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let choix = input.trim();
                    match choix {
                        "1" => self.action_lire(),
                        "2" => self.action_ecrire(),
                        "3" => self.action_modifier(),
                        "4" => self.action_supprimer(),
                        "5" => self.action_lister(),
                        "6" => {
                            println!(" Au revoir!");
                            break;
                        }
                        _ => println!(" Option invalide. Choisissez entre 1 et 6."),
                    }
                }
                Err(e) => {
                    println!(" Erreur de lecture: {}", e);
                }
            }
        }
    }

    fn action_lire(&self) {
        print!(" Nom du fichier à lire: ");
        io::stdout().flush().unwrap();
        
        let mut nom_fichier = String::new();
        if io::stdin().read_line(&mut nom_fichier).is_ok() {
            let nom_fichier = nom_fichier.trim();
            match self.lire_fichier(nom_fichier) {
                Ok(contenu) => {
                    println!("\n Contenu du fichier:");
                    println!("{}", contenu);
                }
                Err(_) => {}
            }
        }
    }

    fn action_ecrire(&self) {
        print!(" Nom du fichier: ");
        io::stdout().flush().unwrap();
        
        let mut nom_fichier = String::new();
        if io::stdin().read_line(&mut nom_fichier).is_ok() {
            let nom_fichier = nom_fichier.trim();
            
            println!(" Entrez le contenu (tapez 'FIN' sur une ligne séparée pour terminer):");
            let mut contenu = String::new();
            
            while let Ok(_) = io::stdin().read_line(&mut contenu) {
                if contenu.trim().ends_with("FIN") {
                    contenu = contenu.trim().strip_suffix("FIN").unwrap().to_string();
                    break;
                }
            }
            
            let _ = self.ecrire_fichier(nom_fichier, &contenu);
        }
    }

    fn action_modifier(&self) {
        print!(" Nom du fichier à modifier: ");
        io::stdout().flush().unwrap();
        
        let mut nom_fichier = String::new();
        if io::stdin().read_line(&mut nom_fichier).is_ok() {
            let nom_fichier = nom_fichier.trim();
            
            match self.lire_fichier(nom_fichier) {
                Ok(contenu_actuel) => {
                    println!("\n Contenu actuel:");
                    println!("{}", contenu_actuel);
                    
                    println!("\n Nouveau contenu (tapez 'FIN' sur une ligne séparée pour terminer):");
                    let mut nouveau_contenu = String::new();
                    
                    while let Ok(_) = io::stdin().read_line(&mut nouveau_contenu) {
                        if nouveau_contenu.trim().ends_with("FIN") {
                            nouveau_contenu = nouveau_contenu.trim().strip_suffix("FIN").unwrap().to_string();
                            break;
                        }
                    }
                    
                    let _ = self.modifier_fichier(nom_fichier, &nouveau_contenu);
                }
                Err(_) => {}
            }
        }
    }

    fn action_supprimer(&self) {
        print!(" Nom du fichier à supprimer: ");
        io::stdout().flush().unwrap();
        
        let mut nom_fichier = String::new();
        if io::stdin().read_line(&mut nom_fichier).is_ok() {
            let nom_fichier = nom_fichier.trim();
            
            print!(" Êtes-vous sûr de vouloir supprimer '{}' définitivement? (oui/non): ", nom_fichier);
            io::stdout().flush().unwrap();
            
            let mut confirmation = String::new();
            if io::stdin().read_line(&mut confirmation).is_ok() {
                let confirmation = confirmation.trim().to_lowercase();
                match confirmation.as_str() {
                    "oui" | "o" | "yes" | "y" => {
                        let _ = self.supprimer_fichier(nom_fichier);
                    }
                    _ => {
                        println!(" Suppression annulée");
                    }
                }
            }
        }
    }

    fn action_lister(&self) {
        match self.lister_fichiers() {
            Ok(fichiers) => {
                if fichiers.is_empty() {
                    println!(" Aucun fichier trouvé dans le répertoire");
                } else {
                    println!(" Fichiers dans le répertoire:");
                    let mut index = 1;
                    for fichier in fichiers {
                        println!("{}.  {}", index, fichier);
                        index += 1;
                    }
                }
            }
            Err(_) => {}
        }
    }
}

fn main() {
    println!(" Démarrage du Gestionnaire de Fichiers");
    
    let gestionnaire = GestionnaireFichiers::nouveau(".".to_string());
    gestionnaire.menu_interactif();
}