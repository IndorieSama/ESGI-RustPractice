# Synthèse du TP1 + main.rs - Concepts Rust Fondamentaux

Ce projet démontre les concepts fondamentaux du langage Rust à travers deux fichiers principaux : `main.rs` (concepts de base) et `tp1.rs` (application bancaire).

## 📁 Structure du Projet

```
TP0/
├── src/
│   ├── main.rs    # Concepts Rust fondamentaux
│   └── tp1.rs     # Application bancaire interactive
├── Cargo.toml     # Configuration du projet
└── README.md      # Ce fichier
```

## 🔧 Comment Exécuter

```bash
# Exécuter main.rs (par défaut)
cargo run

# Exécuter tp1.rs (application bancaire)
cargo run --bin tp1
```

## 📚 Concepts Rust Démontrés

### 1. **Fonctions** (`main.rs`)

```rust
// Fonction avec paramètres et valeur de retour
fn addition(a: i32, b: i32) -> i32 {
    a + b  // Pas de point-virgule = valeur retournée
}

// Fonction avec référence de chaîne
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

**Points clés :**
- `fn` pour définir une fonction
- `&str` pour les références de chaînes de caractères
- `->` pour spécifier le type de retour
- Dernière expression sans `;` est la valeur retournée

### 2. **Variables et Types de Données** (`main.rs`)

```rust
let first_name = "John";           // &str (référence de chaîne)
let age: u32 = 30;                 // Entier non signé 32 bits
let age_signed: i32 = age as i32;  // Conversion de type
let temperature: f64 = 36.6;       // Nombre à virgule flottante
```

**Points clés :**
- Convention `snake_case` pour les variables
- Types explicites avec `:` 
- Conversion avec `as`
- `.to_string()` pour convertir en String

### 3. **Structures de Contrôle** (`main.rs`)

#### Conditions
```rust
if nombre % 2 == 0 {
    println!("{} est un nombre pair.", nombre);
} else {
    println!("{} est un nombre impair.", nombre);
}
```

#### Boucles
```rust
for i in 1..=5 {        // Inclut 5
    println!("Compteur: {}", i);
}

for i in 1..5 {         // Exclut 5
    println!("Compteur: {}", i);
}
```

### 4. **Collections et Itération** (`main.rs`)

```rust
let names = vec!["Alice", "Bob", "Charlie"];
for (index, value) in names.iter().enumerate() {
    println!("Index: {}, Value: {}", index, value);
}
```

**Points clés :**
- `vec![]` macro pour créer des vecteurs
- `.iter()` pour itérer sur une collection
- `.enumerate()` pour obtenir index et valeur

### 5. **Gestion d'Entrée Utilisateur Avancée** (`main.rs`)

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read input");

// Gestion des plages (ex: "1-3")
if trimmed.contains('-') {
    let parts: Vec<&str> = trimmed.split('-').collect();
    // Validation et traitement...
}
```

**Fonctionnalités :**
- Validation des entrées
- Support des sélections multiples (plages)
- Gestion d'erreurs robuste

### 6. **Structures et Implémentations** (`tp1.rs`)

```rust
// Définition de structure
struct CompteBancaire {
    nom: String,
    solde: f64,
}

// Implémentation des méthodes
impl CompteBancaire {
    fn afficher_solde(&self) {  // Référence immutable
        println!("Compte: {}, Solde: {:.2} €", self.nom, self.solde);
    }

    fn retrait(&mut self, montant: f64) {  // Référence mutable
        if self.solde >= montant {
            self.solde -= montant;
            // ...
        }
    }
}
```

**Points clés :**
- `struct` pour définir des types personnalisés
- `impl` pour implémenter des méthodes
- `&self` pour accès en lecture seule
- `&mut self` pour modifications

### 7. **Ownership et Borrowing** (`tp1.rs`)

```rust
let mut comptes = vec![
    CompteBancaire { nom: "Compte Courant".to_string(), solde: 1250.75 },
    CompteBancaire { nom: "Livret A".to_string(), solde: 5400.00 },
];

// Référence immutable pour affichage
for compte in &comptes {
    compte.afficher_solde();
}

// Accès mutable pour modification
comptes[compte_choix].retrait(montant);
```

### 8. **Gestion d'Erreurs avec Pattern Matching** (`tp1.rs`)

```rust
let choix: u32 = match choix.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Veuillez entrer un numéro valide.");
        continue;
    }
};

// Validation avec garde
let compte_choix: usize = match compte_choix_str.trim().parse::<usize>() {
    Ok(num) if num > 0 && num <= comptes.len() => num - 1,
    _ => {
        println!("Choix de compte invalide.");
        continue;
    }
};
```

**Points clés :**
- `match` pour le pattern matching
- `Ok()` et `Err()` pour gérer les résultats
- Gardes avec `if` dans les patterns
- `continue` pour reprendre la boucle

### 9. **Application Interactive Complète** (`tp1.rs`)

L'application bancaire démontre :
- **Menu interactif** avec boucle principale
- **Validation d'entrées** robuste
- **Gestion d'état** mutable des comptes
- **Interface utilisateur** claire et intuitive

#### Fonctionnalités :
1. **Affichage de solde** - Consultation des comptes
2. **Retrait d'argent** - Modification des soldes avec validation
3. **Liste des comptes** - Affichage de tous les comptes
4. **Système de menu** - Navigation entre les options

## 🎯 Concepts Clés Illustrés

| Concept | Fichier | Description |
|---------|---------|-------------|
| **Functions** | `main.rs` | Définition et appel de fonctions |
| **Variables** | `main.rs` | Types, conversion, mutabilité |
| **Control Flow** | `main.rs` | if/else, boucles for |
| **Collections** | `main.rs` | Vecteurs, itération |
| **User Input** | `main.rs` | Lecture et validation avancée |
| **Structs** | `tp1.rs` | Types personnalisés |
| **Methods** | `tp1.rs` | Implémentation de comportements |
| **Ownership** | `tp1.rs` | Références mutables/immutables |
| **Error Handling** | `tp1.rs` | Pattern matching, validation |
| **State Management** | `tp1.rs` | Application interactive |

