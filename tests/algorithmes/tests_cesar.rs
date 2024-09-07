use crypto_sphere::algorithmes::cesar::Cesar;

#[test]
fn test_chiffrement_cesar() {
    let cesar = Cesar { decalage: 3 };
    let texte = "HELLO";
    let resultat = cesar.chiffrer(texte);
    assert_eq!(resultat, "KHOOR");
}

#[test]
fn test_dechiffrement_cesar() {
    let cesar = Cesar { decalage: 3 };
    let texte_chiffre = "KHOOR";
    let resultat = cesar.dechiffrer(texte_chiffre);
    assert_eq!(resultat, "HELLO");
}
