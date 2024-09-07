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
        let mut choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");

        loop {
            match choix_algo.as_str() {
                "1" => {
                    // Chiffre de César
                    let decalage = loop {
                        if choix_cle == "p" {
                            break constants::CLE_CESAR;
                        } else if choix_cle == "m" {
                            let decalage = ask_non_empty_input("Entrez le décalage pour le chiffre de César (nombre entier) :");
                            match verification::verify_caesar_key(&decalage) {
                                Ok(decalage_valide) => break decalage_valide,
                                Err(_) => {
                                    println!("Clé invalide pour César. Réessayez.");
                                    choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                                }
                            }
                        } else {
                            println!("Mode de clé non valide. Réessayez.");
                            choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                        }
                    };
        
                    let cesar = Cesar { decalage };
        
                    if action == "c" {
                        println!("Texte chiffré : {}", cesar.chiffrer(&texte));
                    } else {
                        println!("Texte déchiffré : {}", cesar.dechiffrer(&texte));
                    }
                    break;
                }
        
                "2" => {
                    // Chiffrement XOR
                    let cle = loop {
                        if choix_cle == "p" {
                            break constants::CLE_XOR;
                        } else if choix_cle == "m" {
                            let cle = ask_non_empty_input("Entrez un caractère comme clé pour XOR :");
                            match verification::verify_xor_key(&cle) {
                                Ok(cle_valide) => break cle_valide,
                                Err(_) => {
                                    println!("Clé invalide pour XOR. Réessayez.");
                                    choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                                }
                            }
                        } else {
                            println!("Mode de clé non valide. Réessayez.");
                            choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                        }
                    };
        
                    let xor = Xor { cle };
        
                    if action == "c" {
                        println!("Texte chiffré : {}", xor.chiffrer(&texte));
                    } else {
                        println!("Texte déchiffré : {}", xor.dechiffrer(&texte));
                    }
                    break;
                }
        
                "3" => {
                    // Rail Fence
                    let rails = loop {
                        if choix_cle == "p" {
                            break constants::CLE_RAIL_FENCE;
                        } else if choix_cle == "m" {
                            let rails = ask_non_empty_input("Entrez le nombre de rails pour Rail Fence (nombre entier) :");
                            match verification::verify_rail_fence_key(&rails) {
                                Ok(rails_valide) => break rails_valide,
                                Err(_) => {
                                    println!("Clé invalide pour Rail Fence. Réessayez.");
                                    choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                                }
                            }
                        } else {
                            println!("Mode de clé non valide. Réessayez.");
                            choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                        }
                    };
        
                    let rail_fence = RailFence { rails };
        
                    if action == "c" {
                        println!("Texte chiffré : {}", rail_fence.chiffrer(&texte));
                    } else {
                        println!("Texte déchiffré : {}", rail_fence.dechiffrer(&texte));
                    }
                    break;
                }
        
                "4" => {
                    // Chiffre de Vigenère
                    let cle = loop {
                        if choix_cle == "p" {
                            break constants::CLE_VIGENERE.to_string();
                        } else if choix_cle == "m" {
                            let cle = ask_non_empty_input("Entrez la clé pour le chiffre de Vigenère :");
                            match verification::verify_vigenere_key(&cle) {
                                Ok(cle_valide) => break cle_valide,
                                Err(_) => {
                                    println!("Clé invalide pour Vigenère. Réessayez.");
                                    choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                                }
                            }
                        } else {
                            println!("Mode de clé non valide. Réessayez.");
                            choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                        }
                    };
        
                    let vigenere = Vigenere { cle };
        
                    if action == "c" {
                        println!("Texte chiffré : {}", vigenere.chiffrer(&texte));
                    } else {
                        println!("Texte déchiffré : {}", vigenere.dechiffrer(&texte));
                    }
                    break;
                }
        
                "5" => {
                    // Chiffre de Substitution
                    let cle = loop {
                        if choix_cle == "p" {
                            break constants::CLE_SUBSTITUTION.to_string();
                        } else if choix_cle == "m" {
                            let cle = ask_non_empty_input("Entrez la clé pour le chiffre de Substitution (26 lettres uniques) :");
                            match verification::verify_substitution_key(&cle) {
                                Ok(cle_valide) => break cle_valide,
                                Err(_) => {
                                    println!("Clé invalide pour Substitution. Réessayez.");
                                    choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                                }
                            }
                        } else {
                            println!("Mode de clé non valide. Réessayez.");
                            choix_cle = ask_non_empty_input("Voulez-vous utiliser la clé prédéfinie (P) ou entrer une clé manuellement (M) ?");
                        }
                    };
        
                    let substitution = Substitution { cle };
        
                    if action == "c" {
                        println!("Texte chiffré : {}", substitution.chiffrer(&texte));
                    } else {
                        println!("Texte déchiffré : {}", substitution.dechiffrer(&texte));
                    }
                    break;
                }
        
                _ => {
                    println!("Choix d'algorithme non valide.");
                    break; // Sortir du loop principal si l'algorithme est invalide
                }
            }
        }
        
        
        loop {
            let continuer = ask_non_empty_input("Voulez-vous continuer ? (1 pour Oui, 0 pour Non) :")
                .trim()
                .to_string(); // Convertir l'entrée en String après le trim

            if continuer == "0" {
                println!("Merci d'avoir utilisé notre programme !");
                return; // Terminer le programme
            } else if continuer == "1" {
                break; // Continuer avec un nouvel algorithme
            } else {
                println!("Réponse invalide. Veuillez entrer 1 pour Oui ou 0 pour Non.");
            }
        }
    }
}