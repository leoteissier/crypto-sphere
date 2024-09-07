mod algorithmes;
mod verification;
mod constants;
mod utils;

use utils::functions::ask_non_empty_input;
use algorithmes::{Chiffrement, cesar::Cesar, xor::Xor, rail_fence::RailFence, vigenere::Vigenere, substitution::Substitution};

fn main() {
    loop {
        println!("Choisissez un algorithme de chiffrement :");
        println!("1. Chiffre de César");
        println!("2. Chiffrement XOR");
        println!("3. Rail Fence");
        println!("4. Chiffre de Vigenère");
        println!("5. Chiffre de Substitution");

        let choix_algo = ask_non_empty_input("Veuillez entrer le numéro de l'algorithme :");

        let texte = ask_non_empty_input("Entrez le texte à chiffrer/déchiffrer :");

        let action = ask_non_empty_input("Voulez-vous (C)hiffrer ou (D)échiffrer ?").to_lowercase();

        // Demander si l'utilisateur souhaite utiliser la clé prédéfinie ou entrer une clé manuellement
        let choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?").to_lowercase();

        match choix_algo.as_str() {
            "1" => {
                let decalage = if choix_cle == "p" {
                    constants::CLE_CESAR
                } else {
                    let decalage = ask_non_empty_input("Entrez le décalage pour le chiffre de César :");
                    verification::verify_caesar_key(&decalage).expect("Clé invalide pour César")
                };

                let cesar = Cesar { decalage };

                if action == "c" {
                    println!("Texte chiffré : {}", cesar.chiffrer(&texte));
                } else {
                    println!("Texte déchiffré : {}", cesar.dechiffrer(&texte));
                }
            }

            "2" => {
                let cle = if choix_cle == "p" {
                    constants::CLE_XOR
                } else {
                    let cle = ask_non_empty_input("Entrez un caractère comme clé pour XOR :");
                    verification::verify_xor_key(&cle).expect("Clé invalide pour XOR")
                };

                let xor = Xor { cle };

                if action == "c" {
                    println!("Texte chiffré : {}", xor.chiffrer(&texte));
                } else {
                    println!("Texte déchiffré : {}", xor.dechiffrer(&texte));
                }
            }

            "3" => {
                let rails = if choix_cle == "p" {
                    constants::CLE_RAIL_FENCE
                } else {
                    let rails = ask_non_empty_input("Entrez le nombre de rails pour Rail Fence :");
                    verification::verify_rail_fence_key(&rails).expect("Clé invalide pour Rail Fence")
                };

                let rail_fence = RailFence { rails };

                if action == "c" {
                    println!("Texte chiffré : {}", rail_fence.chiffrer(&texte));
                } else {
                    println!("Texte déchiffré : {}", rail_fence.dechiffrer(&texte));
                }
            }

            "4" => {
                let cle = if choix_cle == "p" {
                    constants::CLE_VIGENERE.to_string()
                } else {
                    let cle = ask_non_empty_input("Entrez la clé pour le chiffre de Vigenère :");
                    verification::verify_vigenere_key(&cle).expect("Clé invalide pour Vigenère")
                };

                let vigenere = Vigenere { cle };

                if action == "c" {
                    println!("Texte chiffré : {}", vigenere.chiffrer(&texte));
                } else {
                    println!("Texte déchiffré : {}", vigenere.dechiffrer(&texte));
                }
            }

            "5" => {
                let cle = if choix_cle == "p" {
                    constants::CLE_SUBSTITUTION.to_string()
                } else {
                    let cle = ask_non_empty_input("Entrez la clé pour le chiffre de Substitution (26 lettres uniques) :");
                    verification::verify_substitution_key(&cle).expect("Clé invalide pour Substitution")
                };

                let substitution = Substitution { cle };

                if action == "c" {
                    println!("Texte chiffré : {}", substitution.chiffrer(&texte));
                } else {
                    println!("Texte déchiffré : {}", substitution.dechiffrer(&texte));
                }
            }

            _ => println!("Choix d'algorithme non valide."),
        }

        let continuer = ask_non_empty_input("Voulez-vous continuer ? (1 pour Oui, 0 pour Non) :");
        if continuer.trim() == "0" {
            break;
        }
    }
}