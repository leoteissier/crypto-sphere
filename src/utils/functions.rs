use std::io::{self};

// Fonction utilitaire pour demander une entrée utilisateur non vide
pub fn ask_non_empty_input(prompt: &str) -> String {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de l'entrée");
        let trimmed_input = input.trim().to_string();

        if !trimmed_input.is_empty() {
            return trimmed_input;
        } else {
            println!("Entrée vide non valide. Veuillez réessayer.");
        }
    }
}