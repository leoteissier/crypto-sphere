use crypto_sphere::algorithmes::vigenere::Vigenere;

#[test]
fn test_chiffrement_vigenere() {
    let vigenere = Vigenere { cle: String::from("KEY") };
    let texte = "HELLO";
    let resultat = vigenere.chiffrer(texte);
    assert_eq!(resultat, "RIJVS"); // Résultat chiffré attendu avec la clé "KEY"
}

#[test]
fn test_dechiffrement_vigenere() {
    let vigenere = Vigenere { cle: String::from("KEY") };
    let texte_chiffre = "RIJVS";
    let resultat = vigenere.dechiffrer(texte_chiffre);
    assert_eq!(resultat, "HELLO");
}