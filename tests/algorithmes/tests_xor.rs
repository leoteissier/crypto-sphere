use crypto_sphere::algorithmes::xor::Xor;

#[test]
fn test_chiffrement_xor() {
    let xor = Xor { cle: b'A' };
    let texte = "HELLO";
    let resultat = xor.chiffrer(texte);
    assert_eq!(resultat, "(\u{0c}\u{0f}\u{0f}\u{0b}\u{0e}");  // Résultat XOR attendu
}

#[test]
fn test_dechiffrement_xor() {
    let xor = Xor { cle: b'A' };
    let texte_chiffre = "(\u{0c}\u{0f}\u{0f}\u{0b}\u{0e}";
    let resultat = xor.dechiffrer(texte_chiffre);
    assert_eq!(resultat, "HELLO");
}
