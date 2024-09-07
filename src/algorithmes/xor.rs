use super::Chiffrement;

pub struct Xor {
    pub cle: u8,
}

impl Chiffrement for Xor {
    fn chiffrer(&self, texte: &str) -> String {
        texte.chars()
            .map(|c| (c as u8 ^ self.cle) as char)
            .collect()
    }

    fn dechiffrer(&self, texte: &str) -> String {
        // Déchiffrement XOR est identique au chiffrement
        self.chiffrer(texte)
    }
}
