pub fn remove_spaces(texte: &str) -> String {
    texte.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn is_valid_text(texte: &str) -> bool {
    // Vérifie que le texte contient uniquement des caractères alphabétiques
    texte.chars().all(|c| c.is_ascii_alphabetic())
}

pub fn clean_input(texte: &str) -> String {
    // Nettoie l'entrée en supprimant les espaces et en mettant le texte en majuscule
    remove_spaces(&texte.to_uppercase())
}
