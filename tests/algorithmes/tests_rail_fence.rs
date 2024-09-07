use crypto_sphere::algorithmes::rail_fence::RailFence;

#[test]
fn test_chiffrement_rail_fence() {
    let rail_fence = RailFence { rails: 3 };
    let texte = "HELLO";
    let resultat = rail_fence.chiffrer(texte);
    assert_eq!(resultat, "HOELL"); // Résultat attendu pour Rail Fence avec 3 rails
}

#[test]
fn test_dechiffrement_rail_fence() {
    let rail_fence = RailFence { rails: 3 };
    let texte_chiffre = "HOELL";
    let resultat = rail_fence.dechiffrer(texte_chiffre);
    assert_eq!(resultat, "HELLO");
}