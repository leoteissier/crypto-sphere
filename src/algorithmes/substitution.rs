use super::Chiffrement;

pub struct Substitution {
    pub cle: String,
}

impl Chiffrement for Substitution {
    fn chiffrer(&self, texte: &str) -> String {
        // Tableau de substitution basé sur la clé
        let mut resultat = String::new();

        for c in texte.chars() {
            if c.is_ascii_alphabetic() {
                let index = c.to_ascii_uppercase() as usize - 'A' as usize;
                resultat.push(self.cle.chars().nth(index).unwrap());
            } else {
                resultat.push(c); // Conserver les caractères non alphabétiques tels quels
            }
        }

        resultat
    }

    fn dechiffrer(&self, texte: &str) -> String {
        // Tableau de déchiffrement inversé basé sur la clé
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut resultat = String::new();

        for c in texte.chars() {
            if c.is_ascii_alphabetic() {
                let index = self.cle.chars().position(|x| x == c.to_ascii_uppercase()).unwrap();
                resultat.push(alphabet.chars().nth(index).unwrap());
            } else {
                resultat.push(c); // Conserver les caractères non alphabétiques tels quels
            }
        }

        resultat
    }
}
