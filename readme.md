# Mon Projet de Cryptographie

[![pipeline status](https://gitlab.com/leoteissier/crypto-sphere/badges/main/pipeline.svg)](https://gitlab.com/leoteissier/crytpo-sphere/commits/main)

## Description

Ce projet est un ensemble d'algorithmes de cryptographie implémentés en **Rust**. Il inclut différents algorithmes de chiffrement classiques tels que :

- Chiffre de César
- Chiffrement XOR
- Chiffre de Vigenère
- Chiffre de Substitution
- Chiffre Rail Fence

Ce projet permet de chiffrer et déchiffrer des textes avec ces algorithmes et peut être facilement étendu pour ajouter de nouveaux algorithmes.

## Prérequis

- **Rust** installé : Suivez les instructions sur [rust-lang.org](https://www.rust-lang.org/tools/install) pour installer Rust.

## Installation

Clonez ce projet depuis GitLab ou GitHub et compilez-le avec **Cargo** :

```bash
git clone https://gitlab.com/leoteissier/crytpo-sphere.git
cd crytpo-sphere
cargo build
```

Utilisation
Une fois compilé, vous pouvez exécuter le programme en utilisant la commande suivante :

```bash
cargo run
```

Fonctionnalités principales
Le programme vous permet de :

- Choisir un algorithme de chiffrement parmi plusieurs options.
- Chiffrer ou déchiffrer un texte donné.
- Utiliser une clé prédéfinie ou entrer votre propre clé pour certains algorithmes.

## Exécution des tests

Ce projet inclut des tests unitaires pour valider le bon fonctionnement des algorithmes. Pour exécuter les tests, utilisez la commande suivante :

```bash
cargo test
```

Les tests sont organisés dans le dossier tests/algorithmes et couvrent les différents algorithmes de chiffrement.

## Intégration Continue (CI)

Ce projet utilise GitLab CI pour automatiser la compilation et l'exécution des tests à chaque push. Le fichier de configuration CI est situé à la racine du projet sous le nom .gitlab-ci.yml.

* Chaque commit déclenche une pipeline qui effectue les actions suivantes :
  - Compilation du projet
  - Exécution des tests unitaires

## Contribution

Les contributions sont les bienvenues ! Pour proposer des modifications :

Forkez le projet.
Créez une branche pour votre fonctionnalité (git checkout -b ma-fonctionnalite).
Poussez la branche sur votre dépôt (git push origin ma-fonctionnalite).
Créez une Merge Request sur GitLab.

## Licence

Ce projet est sous licence MIT. Consultez le fichier LICENSE pour plus de détails
