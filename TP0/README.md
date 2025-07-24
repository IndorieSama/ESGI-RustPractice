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
├── src/
│   ├── fichiers/                   # Dossier contenant les fichiers générés par gestionnaire-fichiers.rs (crée automatiquement)
│   ├── main.rs                     # Concepts fondamentaux
│   ├── gestionnaire-fichiers.rs    # Gestion de fichiers et sérialisation
│   ├── tp1.rs                      # Système bancaire
├── Cargo.lock                      # Dépendances du projet
├── Cargo.toml                      # Configuration du projet
└── README.md                       # Documentation
```

## Programmes disponibles

### 0. Build et exécution
Pour compiler et exécuter les programmes, utilisez les commandes suivantes :
```bash
cargo build
cargo run 
```

ou pour exécuter un binaire spécifique :

```bash
cargo run --bin <nom_du_binaire>
```


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

### 4. Gestionnaire de fichiers (gestionnaire-fichiers.rs)
Outils pour lire et écrire des fichiers, sérialiser et désérialiser des données.
```bash
cargo run --bin gestionnaire-fichiers
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

### Manipulation de fichiers (gestionnaire-fichiers.rs)
- Lecture et écriture de fichiers
- Sérialisation et désérialisation de données
- Utilisation de bibliothèques externes (serde, chrono)
- Traitement de données structurées