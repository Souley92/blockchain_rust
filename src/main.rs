mod block;
mod blockchain;
mod transaction;

use blockchain::Blockchain;
use transaction::NFTTransaction;
use hyper::{Body, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use text_io::read;
use std::fs::File;
use std::path::Path;
use std::io::{self, Write, Read};
use reqwest;

const BLOCKCHAIN_FILE: &str = "blockchain.json";

#[tokio::main]
async fn main() -> io::Result<()> {
    let blockchain = load_blockchain().unwrap_or_else(|| Blockchain::new(4));
    let blockchain = Arc::new(Mutex::new(blockchain));

    let make_svc = make_service_fn(move |_conn| {
        let blockchain = Arc::clone(&blockchain);
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let blockchain = Arc::clone(&blockchain);
                async move {
                    match (req.method(), req.uri().path()) {
                        (&hyper::Method::GET, "/blockchain") => {
                            let blockchain = blockchain.lock().unwrap();
                            let body = serde_json::to_string(&*blockchain).unwrap();
                            Ok::<_, hyper::Error>(Response::new(Body::from(body)))
                        },
                        (&hyper::Method::POST, "/transaction") => {
                            let bytes = hyper::body::to_bytes(req.into_body()).await?;
                            let txn: NFTTransaction = serde_json::from_slice(&bytes).unwrap();
                            let mut blockchain = blockchain.lock().unwrap();
                            blockchain.create_transaction(txn);
                            blockchain.mine_pending_transactions();
                            save_blockchain(&blockchain).unwrap();
                            Ok::<_, hyper::Error>(Response::new(Body::from("Transaction added")))
                        },
                        _ => {
                            Ok::<_, hyper::Error>(Response::new(Body::from("Not found")))
                        }
                    }
                }
            }))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("server error: {}", e);
        }
    });

    // Client loop to add transactions
    loop {
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

        // Envoyer la transaction au serveur local
        match send_transaction(txn).await {
            Ok(response) => println!("Transaction response: {}", response),
            Err(e) => eprintln!("Failed to send transaction: {}", e),
        }
    }
}

async fn send_transaction(txn: NFTTransaction) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:3000/transaction")
        .json(&txn)
        .send()
        .await?;

    res.text().await
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

fn save_blockchain(blockchain: &Blockchain) -> io::Result<()> {
    let serialized = serde_json::to_string(blockchain).expect("Failed to serialize blockchain");
    let mut file = File::create(BLOCKCHAIN_FILE)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn load_blockchain() -> Option<Blockchain> {
    if Path::new(BLOCKCHAIN_FILE).exists() {
        let file = File::open(BLOCKCHAIN_FILE).ok()?;
        let blockchain: Blockchain = serde_json::from_reader(file).expect("Failed to deserialize blockchain");
        Some(blockchain)
    } else {
        None
    }
}
