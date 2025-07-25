# ESGI Rust Practice - Programmation Réseau

Projet d'apprentissage avancé du langage Rust avec focus sur la programmation réseau

# Capture fonctionelle TP 7 à 9

Les captures fonctionnelles des TP 7 à 9 sont disponibles dans le dossier `Captures_Fonctionelle` du projet. 
Elles montrent les interactions avec les clients et serveurs, ainsi que les fonctionnalités implémentées.

# Table des matières
- [Vue d'ensemble](#vue-densemble)
- [Structure du projet](#structure-du-projet)
- [Guide d'utilisation](#guide-dutilisation)
- [Démarrage rapide](#démarrage-rapide)
- [Fonctionnalités détaillées](#fonctionnalités-détaillées)
- [Concepts Rust illustrés](#concepts-rust-illustrés)
- [Concepts de base](#concepts-de-base)
- [Programmation orientée objet](#programmation-orientée-objet)
- [Manipulation de fichiers](#manipulation-de-fichiers)
- [Programmation réseau avancée](#programmation-réseau-avancée)
- [Badges](#badges)



## Vue d'ensemble

Ce projet contient plusieurs programmes Rust démontrant une progression complète en programmation réseau :
- **TP0** : Concepts fondamentaux du langage Rust
- **TP7** : Client et Serveur DNS simples (UDP)
- **TP8** : Protocole de calcul à distance personnalisé (TCP)
- **TP9** : Chat en temps réel avec WebSockets
- **Serveur de journalisation** : TCP avec gestion des connexions concurrentes

### Concepts couverts
- Programmation réseau UDP et TCP
- Protocoles personnalisés avec sérialisation/désérialisation
- Communication WebSocket bidirectionnelle
- Programmation asynchrone avec Tokio
- Gestion des messages texte et binaires
- Accès concurrent sécurisé aux ressources partagées


## Structure du projet

```
ESGI-RustPractice/
├── TP0/                            # Exercices de base Rust
│   ├── src/
│   │   ├── fichiers/               # Fichiers générés par gestionnaire_fichiers.rs
│   │   ├── main.rs                 # Concepts fondamentaux
│   │   ├── gestionnaire_fichiers.rs # Gestion de fichiers et sérialisation
│   │   └── tp1.rs                  # Système bancaire
│   └── Cargo.toml
├── tp7_dns_simple/                 # TP7 - DNS Client/Serveur (UDP)
│   ├── src/
│   │   ├── lib.rs                  # Structures DNS selon RFC 1035
│   │   ├── main.rs                 # Point d'entrée principal
│   │   ├── client.rs               # Client DNS UDP
│   │   └── serveur.rs              # Serveur DNS UDP
│   └── Cargo.toml
├── tp8_protocole_personnalise/     # TP8 - Protocole de calcul personnalisé (TCP)
│   ├── src/
│   │   ├── lib.rs                  # Protocole de calcul JSON avec operations math
│   │   ├── main.rs                 # Point d'entrée principal
│   │   ├── client.rs               # Client de calcul TCP interactif
│   │   └── serveur.rs              # Serveur de calcul TCP multi-sessions
│   └── Cargo.toml
├── tp9_websocket/                  # TP9 - WebSocket Chat (WebSocket)
│   ├── src/
│   │   ├── lib.rs                  # Messages WebSocket texte/binaire
│   │   ├── main.rs                 # Point d'entrée principal
│   │   ├── client.rs               # Client WebSocket interactif
│   │   └── serveur.rs              # Serveur WebSocket multi-connexions
│   └── Cargo.toml
├── journalisation_server/          # Serveur de journalisation TCP
│   ├── src/
│   │   ├── main.rs                 # Serveur TCP
│   │   ├── client.rs               # Client TCP
│   │   └── test_concurrent.rs      # Test de concurrence avec 10 clients
│   ├── logs/
│   │   └── server.log              # Fichier de logs
│   └── Cargo.toml
├── README.md                       # Documentation
└── TASK_REQUIREMENTS.md            # Spécifications des TP
```

## Guide d'utilisation

### Démarrage rapide

Chaque TP est un projet Rust indépendant. Naviguez dans le dossier souhaité et utilisez `cargo run`.

#### TP7 - DNS Client/Serveur (UDP)
```bash
cd tp7_dns_simple

# Terminal 1 - Serveur DNS
cargo run --bin serveur_dns

# Terminal 2 - Client DNS
cargo run --bin client_dns
```

#### TP8 - Protocole de calcul à distance personnalisé (TCP)
```bash
cd tp8_protocole_personnalise

# Terminal 1 - Serveur de calcul
cargo run --bin serveur

# Terminal 2+ - Clients de calcul
cargo run --bin client
```

#### TP9 - Chat WebSocket en temps réel
```bash
cd tp9_websocket

# Terminal 1 - Serveur WebSocket
cargo run --bin serveur_websocket

# Terminal 2+ - Clients WebSocket
cargo run --bin client_websocket
```

#### TP0 - Exercices de base Rust
```bash
cd TP0
cargo run                             # Programme principal
cargo run --bin tp1                   # Système bancaire
cargo run --bin gestionnaire_fichiers # Gestion de fichiers
```

#### Serveur de journalisation
```bash
cd journalisation_server
cargo run --bin server         # Serveur TCP
cargo run --bin client         # Client TCP
cargo run --bin test_concurrent # Test de concurrence (10 clients)
```

## Fonctionnalités détaillées

### TP7 - DNS Client/Serveur (UDP)
- Client DNS : Résolution de noms de domaine en adresses IP
- Serveur DNS : Réponses à des requêtes pour domaines prédéfinis
- Protocole UDP : Communication réseau non-connectée
- Format DNS RFC 1035 : Parsing et construction de messages DNS
- Interface interactive : Session de résolution en temps réel

### TP8 - Protocole de calcul à distance personnalisé (TCP)
- Protocole personnalisé : Format JSON avec sérialisation serde pour calculs
- Serveur de calcul : Gestion simultanée de multiples sessions client
- Operations mathématiques : Addition, Soustraction, Multiplication, Division, Puissance, Racine, Factorielle, Fibonacci
- Gestion des sessions : Suivi des statistiques par client avec ID de session
- Commandes serveur : info (informations), stats (statistiques), ping (test connexion)
- Robustesse : Validation des paramètres et gestion complète des erreurs

### TP9 - Chat WebSocket en temps réel
- Serveur WebSocket : Multi-connexions avec tokio-tungstenite
- Handshake automatique : Établissement de connexion WebSocket
- Communication full-duplex : Bidirectionnelle et persistante
- Messages texte/binaires : Support des deux formats WebSocket
- Chat temps réel : Diffusion instantanée des messages
- Simulation de fichiers : Envoi de données binaires simulées
- Commandes avancées : /help, /users, /stats, /ping, /file

### Projets de base
- TP0 : Concepts fondamentaux Rust et système bancaire
- Serveur de journalisation : TCP avec concurrence et logging

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

### Programmation réseau avancée
- **UDP (TP7)** : Client/Serveur DNS avec protocole RFC 1035
- **TCP (TP8)** : Protocole personnalisé de calcul à distance avec JSON
- **WebSocket (TP9)** : Chat temps réel avec messages texte/binaires
- **Serveur de journalisation** : TCP asynchrone avec Tokio
- **Concurrence** : Arc, Mutex et gestion multi-clients
- **Sérialisation** : serde pour JSON et données binaires
- **Programmation asynchrone** : tokio et futures pour performance


# Badges

![Rust](https://img.shields.io/badge/rust-000000?style=flat-square&logo=rust&logoColor=white)
![GitHub last commit](https://img.shields.io/github/last-commit/IndorieSama/ESGI-RustPractice?style=flat-square)
![GitHub repo size](https://img.shields.io/github/repo-size/IndorieSama/ESGI-RustPractice?style=flat-square)
![GitHub language count](https://img.shields.io/github/languages/count/IndorieSama/ESGI-RustPractice?style=flat-square)
![GitHub top language](https://img.shields.io/github/languages/top/IndorieSama/ESGI-RustPractice?style=flat-square)