# ESGI Rust Practice

Projet d'apprentissage du langage Rust avec manipulation de fichiers et gestion de trames structurées.

## Vue d'ensemble

Ce projet contient plusieurs programmes Rust démontrant :
- Concepts fondamentaux du langage Rust
- Système bancaire interactif avec menu
- Génération et analyse de trames de données structurées
- Écriture dans plusieurs formats de fichiers

## Programmes disponibles

### 1. Programme principal (main.rs)
Démonstration des concepts de base de Rust : variables, fonctions, conditions, boucles.
```bash
cargo run
```

### 2. Système bancaire (tp1.rs)
Application interactive de gestion de comptes bancaires avec menu complet.
```bash
cargo run --bin tp1
```

### 3. Générateur de trames (ecrivain.rs)
Génère des trames de données structurées dans deux formats simultanément :
- Format .log (technique avec timestamps Unix)
- Format .txt (lisible avec dates formatées)
```bash
cargo run --bin ecrivain
```

### 4. Analyseur de trames (analyseur_trame.rs)
Analyse les fichiers de log générés et produit des statistiques détaillées.
```bash
cargo run --bin analyseur_trame
```

## Système de trames

### Structure des trames
Chaque entrée contient :
- **Timestamp** : Horodatage Unix
- **Niveau** : Type d'événement (INFO, ERROR, WARNING, TRANSACTION, SECURITY, etc.)
- **Message** : Description de l'événement
- **Données** : Informations complémentaires (optionnel)

### Formats de sortie

#### Format .log (technique)
```
[1753116791] INFO - Démarrage de l'application
[1753116791] TRANSACTION - Création de compte | Data: Compte Courant - Solde initial: 1500.00€
[1753116791] ERROR - Échec de validation | Data: Code erreur: ERR_001
```

#### Format .txt (lisible)
```
2025-07-21 23:11:11 [INFO] Démarrage de l'application
2025-07-21 23:11:11 [TRANSACTION] Création de compte - Données: Compte Courant - Solde initial: 1500.00€
2025-07-21 23:11:11 [ERROR] Échec de validation - Données: Code erreur: ERR_001
```

### Types de trames générées

1. **session_principale** : Événements généraux de l'application
   - Démarrage/arrêt de l'application
   - Transactions bancaires
   - Erreurs et avertissements

2. **securite** : Événements de sécurité
   - Connexions/déconnexions
   - Authentifications
   - Tentatives d'accès non autorisé

3. **temps_reel** : Événements temps réel
   - Communications WebSocket
   - Réceptions de données
   - Traitements en cours

## Utilisation

### Workflow complet
```bash
# 1. Générer les trames de données
cargo run --bin ecrivain

# 2. Analyser les trames générées
cargo run --bin analyseur_trame

# 3. Tester le système bancaire
cargo run --bin tp1
```

### Compilation de tous les binaires
```bash
cargo build --bins
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

### Manipulation de fichiers (ecrivain.rs, analyseur_trame.rs)
- Écriture de fichiers avec différents formats
- Lecture et parsing de fichiers structurés
- Gestion des timestamps et dates
- Structures de données complexes

## Structure du projet

```
ESGI-RustPractice/
├── src/
│   ├── main.rs               # Concepts fondamentaux
│   ├── tp1.rs                # Système bancaire
│   ├── ecrivain.rs           # Générateur de trames
│   ├── analyseur_trame.rs    # Analyseur de logs
│   ├── session_principale.log  # Trame principale (format technique)
│   ├── session_principale.txt  # Trame principale (format lisible)
│   ├── securite.log           # Trame sécurité (format technique)
│   ├── securite.txt           # Trame sécurité (format lisible)
│   ├── temps_reel.log         # Trame temps réel (format technique)
│   └── temps_reel.txt         # Trame temps réel (format lisible)
├── Cargo.toml               # Configuration du projet
└── README.md               # Documentation
```

## Fonctionnalités avancées

- **Double format automatique** : Chaque trame est écrite simultanément en .log et .txt
- **Analyse statistique** : Comptage par niveau, détection automatique d'erreurs
- **Filtrage intelligent** : Séparation par types d'événements
- **Validation robuste** : Gestion d'erreurs et récupération
- **Formats optimisés** : Technique pour les machines, lisible pour les humains

## Apprentissage progressif

1. **Débutant** : Commencer par `cargo run` pour voir les concepts de base
2. **Intermédiaire** : Tester `cargo run --bin tp1` pour l'application complète
3. **Avancé** : Utiliser `cargo run --bin ecrivain` puis `cargo run --bin analyseur_trame` pour la manipulation de fichiers

