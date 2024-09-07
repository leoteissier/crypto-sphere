use crypto_sphere::algorithmes::substitution::Substitution;

#[test]
fn test_chiffrement_substitution() {
    let cle = String::from("QWERTYUIOPLKJHGFDSAZXCVBNM");
    let substitution = Substitution { cle };
    let texte = "HELLO";
    let resultat = substitution.chiffrer(texte);
    assert_eq!(resultat, "ITSSG"); // Résultat attendu avec la clé de substitution
}

#[test]
fn test_dechiffrement_substitution() {
    let cle = String::from("QWERTYUIOPLKJHGFDSAZXCVBNM");
    let substitution = Substitution { cle };
    let texte_chiffre = "ITSSG";
    let resultat = substitution.dechiffrer(texte_chiffre);
    assert_eq!(resultat, "HELLO");
}
