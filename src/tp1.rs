use std::io;

// Définit la structure pour un compte bancaire
struct CompteBancaire {
    nom: String,
    solde: f64,
}

// Implémente les méthodes pour la structure CompteBancaire
impl CompteBancaire {
    // Affiche le solde du compte
    fn afficher_solde(&self) {
        println!("Compte: {}, Solde: {:.2} €", self.nom, self.solde);
    }

    // Effectue un retrait sur le compte
    fn retrait(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Le montant du retrait doit être positif.");
        } else if self.solde >= montant {
            self.solde -= montant;
            println!("Retrait de {:.2} € effectué.", montant);
            self.afficher_solde();
        } else {
            println!("Solde insuffisant pour un retrait de {:.2} €.", montant);
        }
    }

    // BONUS: Effectue un dépôt sur le compte (empêche les montants négatifs)
    fn depot(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Le montant du dépôt doit être positif.");
        } else {
            self.solde += montant;
            println!("Dépôt de {:.2} € effectué.", montant);
            self.afficher_solde();
        }
    }

    // BONUS: Renomme le compte et retourne un nouveau compte avec le nom changé
    fn renommer(&self, nouveau_nom: String) -> CompteBancaire {
        CompteBancaire {
            nom: nouveau_nom,
            solde: self.solde,
        }
    }
}

fn main() {
    // Crée une liste de comptes bancaires
    let mut comptes = vec![
        CompteBancaire { nom: "Compte Courant".to_string(), solde: 1250.75 },
        CompteBancaire { nom: "Livret A".to_string(), solde: 5400.00 },
    ];

    // Définit les options du menu
    let options = ["Afficher solde", "Retrait", "Dépôt", "Liste comptes", "Renommer compte", "Quitter"];

    // Boucle principale du menu
    loop {
        println!("\n--- Menu Banque ---");
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }
        println!("--------------------");
        println!("Veuillez choisir une option:");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Échec de la lecture de l'entrée");

        // Valide l'entrée de l'utilisateur
        let choix: u32 = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez entrer un numéro valide.");
                continue;
            }
        };

        // Gère le choix de l'utilisateur
        match choix {
            1 => { // Afficher solde
                println!("De quel compte souhaitez-vous afficher le solde ?");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}. {}", i + 1, compte.nom);
                }
                let mut compte_choix_str = String::new();
                io::stdin().read_line(&mut compte_choix_str).expect("Échec de la lecture");
                let compte_choix: usize = match compte_choix_str.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= comptes.len() => num - 1,
                    _ => {
                        println!("Choix de compte invalide.");
                        continue;
                    }
                };
                comptes[compte_choix].afficher_solde();
            }
            2 => { // Retrait
                println!("De quel compte souhaitez-vous faire un retrait ?");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}. {}", i + 1, compte.nom);
                }
                let mut compte_choix_str = String::new();
                io::stdin().read_line(&mut compte_choix_str).expect("Échec de la lecture");
                let compte_choix: usize = match compte_choix_str.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= comptes.len() => num - 1,
                    _ => {
                        println!("Choix de compte invalide.");
                        continue;
                    }
                };

                println!("Entrez le montant du retrait :");
                let mut montant_str = String::new();
                io::stdin().read_line(&mut montant_str).expect("Échec de la lecture");
                let montant: f64 = match montant_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Montant invalide.");
                        continue;
                    }
                };
                comptes[compte_choix].retrait(montant);
            }
            3 => { // Dépôt
                println!("Sur quel compte souhaitez-vous faire un dépôt ?");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}. {}", i + 1, compte.nom);
                }
                let mut compte_choix_str = String::new();
                io::stdin().read_line(&mut compte_choix_str).expect("Échec de la lecture");
                let compte_choix: usize = match compte_choix_str.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= comptes.len() => num - 1,
                    _ => {
                        println!("Choix de compte invalide.");
                        continue;
                    }
                };

                println!("Entrez le montant du dépôt :");
                let mut montant_str = String::new();
                io::stdin().read_line(&mut montant_str).expect("Échec de la lecture");
                let montant: f64 = match montant_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Montant invalide.");
                        continue;
                    }
                };
                comptes[compte_choix].depot(montant);
            }
            4 => { // Liste comptes
                println!("\n--- Liste des Comptes ---");
                for compte in &comptes {
                    compte.afficher_solde();
                }
                println!("-------------------------");
            }
            5 => { // Renommer compte
                println!("Quel compte souhaitez-vous renommer ?");
                for (i, compte) in comptes.iter().enumerate() {
                    println!("{}. {}", i + 1, compte.nom);
                }
                let mut compte_choix_str = String::new();
                io::stdin().read_line(&mut compte_choix_str).expect("Échec de la lecture");
                let compte_choix: usize = match compte_choix_str.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= comptes.len() => num - 1,
                    _ => {
                        println!("Choix de compte invalide.");
                        continue;
                    }
                };

                println!("Entrez le nouveau nom du compte :");
                let mut nouveau_nom = String::new();
                io::stdin().read_line(&mut nouveau_nom).expect("Échec de la lecture");
                let nouveau_nom = nouveau_nom.trim().to_string();
                
                if nouveau_nom.is_empty() {
                    println!("Le nom ne peut pas être vide.");
                    continue;
                }

                // Crée un nouveau compte avec le nom changé
                let nouveau_compte = comptes[compte_choix].renommer(nouveau_nom.clone());
                comptes[compte_choix] = nouveau_compte;
                println!("Compte renommé avec succès en '{}'.", nouveau_nom);
            }
            6 => { // Quitter
                println!("Merci d'avoir utilisé nos services. À bientôt !");
                break;
            }
            _ => {
                println!("Option invalide. Veuillez choisir une option entre 1 et {}.", options.len());
            }
        }
    }
}
