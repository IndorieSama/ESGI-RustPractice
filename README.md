# ESGI Rust Practice

Projet d'apprentissage du langage Rust avec manipulation de fichiers et gestion de trames structurées.

## Vue d'ensemble

Ce projet contient plusieurs programmes Rust démontrant :
- Concepts fondamentaux du langage Rust
- Système bancaire interactif avec menu
- Génération et analyse de trames de données structurées
- Écriture dans plusieurs formats de fichiers
- Lecture et écriture de fichiers avec sérialisation
- Utilisation de bibliothèques externes pour la manipulation de données
- Gestion des erreurs et validation des données

## Structure du projet

```
ESGI-RustPractice/
├── TP0/                            # Exercices de base Rust
│   ├── src/
│   │   ├── fichiers/               # Fichiers générés par gestionnaire_fichiers.rs
│   │   ├── main.rs                 # Concepts fondamentaux
│   │   ├── gestionnaire_fichiers.rs # Gestion de fichiers et sérialisation
│   │   ├── tp1.rs                  # Système bancaire
│   │   └── goodPractice.rs         # Bonnes pratiques Rust
│   ├── Cargo.lock
│   └── Cargo.toml
├── journalisation_server/          # Serveur de journalisation TCP
│   ├── src/
│   │   ├── main.rs                 # Serveur TCP
│   │   └── client.rs               # Client TCP
│   ├── logs/
│   │   └── server.log              # Fichier de logs
│   ├── Cargo.lock
│   └── Cargo.toml
└── README.md                       # Documentation
```

## Programmes disponibles

### 0. Build et exécution

#### TP0 - Exercices de base
Pour compiler et exécuter les programmes du TP0 :
```bash
cd TP0
cargo build
cargo run  # Programme principal (main.rs)
```

ou pour exécuter un binaire spécifique :
```bash
cargo run --bin tp1                    # Système bancaire
cargo run --bin gestionnaire_fichiers  # Gestionnaire de fichiers
```

#### Serveur de journalisation
Pour compiler et exécuter le serveur de journalisation :
```bash
cd journalisation_server
cargo build
cargo run --bin server  # Démarre le serveur TCP
cargo run --bin client  # Démarre le client TCP
```

### 1. Programme principal (TP0/src/main.rs)
Démonstration des concepts de base de Rust : variables, fonctions, conditions, boucles.
```bash
cd TP0 && cargo run
```

### 2. Système bancaire (TP0/src/tp1.rs)
Application interactive de gestion de comptes bancaires avec menu complet.
```bash
cd TP0 && cargo run --bin tp1
```

### 3. Gestionnaire de fichiers (TP0/src/gestionnaire_fichiers.rs)
Outils pour lire et écrire des fichiers, sérialiser et désérialiser des données.
```bash
cd TP0 && cargo run --bin gestionnaire_fichiers
```

### 4. Serveur de journalisation TCP (journalisation_server/)
Serveur TCP avec client pour la journalisation des événements.
```bash
# Terminal 1 - Serveur
cd journalisation_server && cargo run --bin server

# Terminal 2 - Client
cd journalisation_server && cargo run --bin client
```

## Concepts Rust illustrés

### Concepts de base (main.rs)
- Définition et appel de fonctions
- Types de données et conversion
- Structures de contrôle (if/else, boucles)
- Collections et itération
- Gestion des entrées utilisateur

### Programmation orientée objet (tp1.rs)
- Structures (struct) et implémentations (impl)
- Méthodes avec &self et &mut self
- Ownership et borrowing
- Gestion d'erreurs avec match

### Manipulation de fichiers (gestionnaire_fichiers.rs)
- Lecture et écriture de fichiers
- Sérialisation et désérialisation de données
- Utilisation de bibliothèques externes (chrono)
- Traitement de données structurées

### Programmation réseau (journalisation_server)
- Serveur TCP asynchrone avec Tokio
- Gestion de connexions clients multiples
- Journalisation des événements
- Communication client-serveur en temps réel
- Safety avec Arc et Mutex pour l'accès concurrent aux ressources partagées
