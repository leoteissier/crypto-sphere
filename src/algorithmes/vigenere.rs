use super::Chiffrement;

pub struct Vigenere {
    pub cle: String,
}

impl Chiffrement for Vigenere {
    fn chiffrer(&self, texte: &str) -> String {
        let cle_chars: Vec<char> = self.cle.chars().collect();
        let mut cle_index = 0;

        texte.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    let decalage = cle_chars[cle_index % cle_chars.len()].to_ascii_lowercase() as u8 - b'a';
                    cle_index += 1;
                    ((c as u8 - base + decalage) % 26 + base) as char
                } else {
                    c
                }
            })
            .collect()
    }

    fn dechiffrer(&self, texte: &str) -> String {
        let cle_chars: Vec<char> = self.cle.chars().collect();
        let mut cle_index = 0;

        texte.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    let decalage = cle_chars[cle_index % cle_chars.len()].to_ascii_lowercase() as u8 - b'a';
                    cle_index += 1;
                    ((c as u8 - base + 26 - decalage) % 26 + base) as char
                } else {
                    c
                }
            })
            .collect()
    }
}
