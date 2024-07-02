Blockchain Rust
Description

Ce projet implémente une blockchain simple en utilisant le langage de programmation Rust. Il permet aux utilisateurs de créer et de gérer des transactions NFT (Non-Fungible Token) qui incluent un prénom, un nom, un numéro de classe et le hachage d'une image. La blockchain est sauvegardée sur disque et peut être partagée entre plusieurs nœuds pour créer une base de données décentralisée.
Fonctionnalités

    Création de transactions NFT : Les utilisateurs peuvent ajouter des transactions incluant des informations personnelles et le hachage d'une image.
    Chaîne de blocs (blockchain) : Chaque transaction est ajoutée à un bloc qui est ensuite ajouté à la blockchain.
    Persistance des données : La blockchain est sauvegardée sur disque et peut être rechargée lors du démarrage du programme.
    Serveur HTTP : Le projet inclut un serveur HTTP qui permet d'interagir avec la blockchain via des requêtes GET et POST.
    Décentralisation : La blockchain peut être partagée et synchronisée entre plusieurs nœuds pour créer une base de données décentralisée.

Prérequis

    Rust : Assurez-vous d'avoir Rust et Cargo installés sur votre machine. Vous pouvez les installer depuis rustup.rs.
    Internet : Une connexion Internet est nécessaire pour télécharger les dépendances du projet.

Installation

    Clonez le repository du projet :

    sh

git clone https://github.com/votre-nom-utilisateur/blockchain_rust.git
cd blockchain_rust

Ajoutez les dépendances au fichier Cargo.toml :

toml

[package]
name = "blockchain_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
sha2 = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
text_io = "0.1.8"
hyper = { version = "0.14", features = ["full"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }

Compilez le projet :

sh

    cargo build

Utilisation
Lancement du serveur

Pour lancer le serveur, exécutez la commande suivante :

sh

cargo run

Le serveur écoutera sur http://127.0.0.1:3000.
Ajouter une transaction

    Démarrage du client : Lorsque vous exécutez cargo run, le programme démarre et vous demandera d'entrer les informations nécessaires pour créer une transaction NFT :

    plaintext

    Enter first name:
    Enter last name:
    Enter class number:
    Enter the path to your image:

    Envoi de la transaction : Le client enverra la transaction au serveur local. Vous recevrez une réponse confirmant l'ajout de la transaction.

Consulter la blockchain

Vous pouvez consulter l'état actuel de la blockchain en effectuant une requête GET à l'URL suivante :

sh

curl http://127.0.0.1:3000/blockchain

Cette requête retournera la blockchain actuelle au format JSON.
Exemple de transaction

Voici un exemple de ce à quoi pourrait ressembler une transaction ajoutée à la blockchain :

json

{
  "index": 1,
  "timestamp": 1622473441000,
  "transactions": [
    {
      "first_name": "John",
      "last_name": "Doe",
      "class_number": "101",
      "image_hash": "46f731f0378ba9f90d2204e0286b6952210b1b32090c3ff55641a786165531fc"
    }
  ],
  "previous_hash": "78dc64fb483c96b9c785a8a5b3a740f2e4575029b0f77b33e8a2164ac2480fcc",
  "hash": "0000c2bd107a001f6302f590f00d8599e65d79e905a012793161d53abcfbb9e2",
  "nonce": 65092
}

Structure du projet

    main.rs : Le fichier principal qui contient la logique du serveur HTTP et du client.
    block.rs : Définit la structure et les méthodes pour un bloc de la blockchain.
    blockchain.rs : Définit la structure et les méthodes pour la blockchain.
    transaction.rs : Définit la structure et les méthodes pour une transaction NFT.

Contribution

Les contributions sont les bienvenues. Si vous avez des idées d'améliorations ou des fonctionnalités à ajouter, n'hésitez pas à créer une pull request.
Licence

Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.
