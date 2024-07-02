mod block;
mod blockchain;
mod transaction;

use blockchain::Blockchain;
use transaction::NFTTransaction;
use std::io;
use text_io::read;

fn main() -> io::Result<()> {
    let mut blockchain = Blockchain::new(4);

    // Lire les informations de l'utilisateur
    println!("Enter first name:");
    let first_name: String = read!();
    
    println!("Enter last name:");
    let last_name: String = read!();
    
    println!("Enter class number:");
    let class_number: String = read!();
    
    println!("Enter the path to your image:");
    let image_path: String = read!();

    // Calculer le hachage de l'image
    let image_hash = hash_image(&image_path)?;
    
    // Créer une transaction NFT avec les informations fournies
    let txn = NFTTransaction::new(
        first_name,
        last_name,
        class_number,
        image_hash,
    );

    // Ajouter la transaction à la blockchain
    blockchain.create_transaction(txn);
    blockchain.mine_pending_transactions();

    // Afficher les blocs de la blockchain
    for block in &blockchain.chain {
        println!("{}", block);
        display_transactions(&block.transactions);
    }

    // Vérifier la validité de la blockchain
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());

    Ok(())
}

fn display_transactions(transactions: &Vec<NFTTransaction>) {
    for transaction in transactions {
        println!("Transaction:");
        println!("  First Name: {}", transaction.first_name);
        println!("  Last Name: {}", transaction.last_name);
        println!("  Class Number: {}", transaction.class_number);
        println!("  Image Hash: {}", transaction.image_hash);
    }
}

use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;

fn hash_image(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
