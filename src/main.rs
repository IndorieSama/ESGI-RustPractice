//2 les fonctions : 
// fn définir une fonction
// &str pour une référence à une chaîne de caractères
// fonction addition qui retourne la somme de deux entiers
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // utilise le snacke_case pour les variables
    let first_name = "John";
    let age: u32 = 30;
    let age_str = age.to_string();
    println!("Your first name is: {}", first_name);
    println!("Your age is: {}", age_str);
    // signed value
    let age_signed: i32 = age as i32;
    let age_signed_str = age_signed.to_string();
    println!("Your age as signed: {}", age_signed_str);

    let temperature: f64 = 36.6;
    let temperature_str = temperature.to_string();
    println!("Your temperature is: {}", temperature_str);

    // appel de la fonction addition
    let sum: i32 = addition(5, 10);
    println!("The sum of 5 and 10 is: {}", sum);

    // appel de la fonction greet
    greet("Alice");

    // 3 les conditions et les boucles

    let nombre = 10;
    if nombre % 2 == 0 {
        println!("{} est un nombre pair.", nombre);
    } else {
        println!("{} est un nombre impair.", nombre);
    }

    for i in 1..=5 {
        println!("Compteur: {}", i);
    }

    // à noter que 1..=5 inclut 5, tandis que 1..5 n'inclut pas 5

    for i in 1..5 {
        println!("Compteur: {}", i);
    }

    // Itérer sur des références à des éléments d'un tableau
    let tab = [1, 2, 3, 4, 5];
    for &element in &tab {
        println!("Élément du tableau: {}", element);
    }

    //loop
    let mut compteur = 0;
    loop {
        compteur += 1;
        if compteur > 5 {
            break;
        }
        println!("Compteur dans la boucle: {}", compteur);
    }

    // while
    let mut compteur_while = 0;
    while compteur_while < 5 {
        compteur_while += 1;
        println!("Compteur dans la boucle while: {}", compteur_while);
    }

    // for (index, value) in collection.iter().enumerate() {
    //     println!("Index: {}, Value: {}", index, value);
    // }

    let names = vec!["Alice", "Bob", "Charlie"];
    for (index, value) in names.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    // Example using enumerate with a collection and std::io with type safety
    use std::io;
    
    // Create a vector of options to choose from
    let options: Vec<&str> = vec![
        "Option 1",
        "Option 2", 
        "Option 3",
    ];
    
    // Display all options with their indices using enumerate
    for (index, option) in options.iter().enumerate() {
        println!("{}: {}", index + 1, option);
    }
    
    // Get user input with proper type safety
    println!("Please enter the number of your choice (1-{}):", options.len());
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let trimmed = input.trim();
    
    // Check if input contains a range (e.g., "1-2")
    if trimmed.contains('-') {
        let parts: Vec<&str> = trimmed.split('-').collect();
        if parts.len() == 2 {
            match (parts[0].trim().parse::<usize>(), parts[1].trim().parse::<usize>()) {
                (Ok(start), Ok(end)) if start >= 1 && end <= options.len() && start <= end => {
                    println!("You selected a range: ");
                    for i in start..=end {
                        println!("- {}", options[i-1]);
                    }
                    return;
                },
                _ => println!("Invalid range! Defaulting to option 1.")
            }
        } else {
            println!("Invalid range format! Defaulting to option 1.");
        }
        println!("You selected: {}", options[0]);
    } else {
        // Original single-option code
        let choice: usize = match trimmed.parse::<usize>() {
            Ok(num) if num >= 1 && num <= options.len() => num,
            Ok(_) => {
                println!("Input out of range! Defaulting to option 1.");
                1
            },
            Err(_) => {
                println!("Invalid input! Defaulting to option 1.");
                1
            }
        };
        
        // Use the validated choice (adjusting for 0-based indexing)
        println!("You selected: {}", options[choice - 1]);
    }


    // 4 les structures de données

    struct Salarie {
        nom: String,
        ville: String,
        age: u32,
        solde: f64,
   }

   let kevin = Salarie {
       nom: String::from("Kevin"),
       ville: String::from("Paris"),
       age: 30,
       solde: 2500.75,
   };

    println!("Nom: {}, Ville: {}, Âge: {}, Solde: {}", kevin.nom, kevin.ville, kevin.age, kevin.solde);


    
}
