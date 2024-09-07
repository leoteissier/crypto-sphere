pub mod cesar;
pub mod vigenere;
pub mod substitution;
pub mod xor;
pub mod rail_fence;

pub trait Chiffrement {
    fn chiffrer(&self, texte: &str) -> String;
    fn dechiffrer(&self, texte: &str) -> String;
}
