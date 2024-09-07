pub fn verify_caesar_key(key: &str) -> Result<i32, &'static str> {
    // La clé pour César doit être un entier
    match key.trim().parse::<i32>() {
        Ok(decalage) => Ok(decalage),
        Err(_) => Err("La clé pour le chiffre de César doit être un nombre entier."),
    }
}

pub fn verify_xor_key(key: &str) -> Result<u8, &'static str> {
    // La clé pour XOR doit être un caractère unique
    if key.len() == 1 {
        Ok(key.chars().next().unwrap() as u8)
    } else {
        Err("La clé pour XOR doit être un seul caractère.")
    }
}

pub fn verify_rail_fence_key(key: &str) -> Result<usize, &'static str> {
    // La clé pour Rail Fence doit être un nombre entier positif
    match key.trim().parse::<usize>() {
        Ok(rails) if rails > 1 => Ok(rails),
        _ => Err("La clé pour Rail Fence doit être un nombre entier positif supérieur à 1."),
    }
}

pub fn verify_vigenere_key(key: &str) -> Result<String, &'static str> {
    // Vérifier que la clé contient uniquement des lettres alphabétiques
    if key.chars().all(|c| c.is_ascii_alphabetic()) {
        Ok(key.to_string())
    } else {
        Err("La clé pour le chiffre de Vigenère doit contenir uniquement des lettres alphabétiques.")
    }
}

pub fn verify_substitution_key(key: &str) -> Result<String, &'static str> {
    let key = key.trim();
    if key.len() != 26 {
        return Err("La clé pour le chiffre de substitution doit contenir exactement 26 caractères.");
    }
    if !key.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err("La clé pour le chiffre de substitution doit contenir uniquement des lettres alphabétiques.");
    }

    // Vérifier que chaque lettre est unique
    let mut seen = [false; 26];
    for c in key.chars() {
        let index = c.to_ascii_lowercase() as usize - 'a' as usize;
        if seen[index] {
            return Err("La clé pour le chiffre de substitution doit contenir des lettres uniques.");
        }
        seen[index] = true;
    }

    Ok(key.to_string())
}
