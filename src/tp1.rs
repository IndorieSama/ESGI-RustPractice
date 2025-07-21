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
}

fn main() {
    // Crée une liste de comptes bancaires
    let mut comptes = vec![
        CompteBancaire { nom: "Compte Courant".to_string(), solde: 1250.75 },
        CompteBancaire { nom: "Livret A".to_string(), solde: 5400.00 },
    ];

    // Définit les options du menu
    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

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
            3 => { // Liste comptes
                println!("\n--- Liste des Comptes ---");
                for compte in &comptes {
                    compte.afficher_solde();
                }
                println!("-------------------------");
            }
            4 => { // Quitter
                println!("Merci d'avoir utilisé nos services. À bientôt !");
                break;
            }
            _ => {
                println!("Option invalide. Veuillez choisir une option entre 1 et {}.", options.len());
            }
        }
    }
}
