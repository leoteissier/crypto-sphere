use super::Chiffrement;

pub struct Cesar {
    pub decalage: i32,
}

impl Chiffrement for Cesar {
    fn chiffrer(&self, texte: &str) -> String {
        texte.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    ((c as u8 - base + self.decalage as u8) % 26 + base) as char
                } else {
                    c
                }
            })
            .collect()
    }

    fn dechiffrer(&self, texte: &str) -> String {
        texte.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                    ((c as u8 - base - self.decalage as u8) % 26 + base) as char
                } else {
                    c
                }
            })
            .collect()
    }
}
