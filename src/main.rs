use sha2::{Sha256, Digest};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use hyper::{Body, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use tokio;
use std::fs::File;
use std::io::Read;
use text_io::read;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct NFTTransaction {
    first_name: String,
    last_name: String,
    class_number: String,
    image_hash: String,
}

impl NFTTransaction {
    fn new(first_name: String, last_name: String, class_number: String, image_hash: String) -> Self {
        NFTTransaction {
            first_name,
            last_name,
            class_number,
            image_hash,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: u32,
    timestamp: i64,
    previous_hash: String,
    hash: String,
    data: Vec<NFTTransaction>,
    nonce: u64,
    difficulty: usize,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: Vec<NFTTransaction>, difficulty: usize) -> Block {
        let timestamp = Utc::now().timestamp_millis();
        let mut block = Block {
            index,
            timestamp,
            previous_hash: previous_hash.clone(),
            hash: String::new(),
            data,
            nonce: 0,
            difficulty,
        };
        block.mine();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(&self.index.to_string());
        hasher.update(&self.timestamp.to_string());
        hasher.update(&self.previous_hash);
        hasher.update(&self.nonce.to_string());
        for txn in &self.data {
            hasher.update(&txn.first_name);
            hasher.update(&txn.last_name);
            hasher.update(&txn.class_number);
            hasher.update(&txn.image_hash);
        }
        format!("{:x}", hasher.finalize())
    }

    fn mine(&mut self) {
        while &self.hash[..self.difficulty] != "0".repeat(self.difficulty) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Blockchain {
        let difficulty = 2; // Ajustez la difficulté selon vos besoins
        let genesis_block = Block::new(0, String::from("0"), vec![], difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    fn add_block(&mut self, data: Vec<NFTTransaction>) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(previous_block.index + 1, previous_block.hash.clone(), data, self.difficulty);
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
            if &current_block.hash[..current_block.difficulty] != "0".repeat(current_block.difficulty) {
                return false;
            }
        }
        true
    }

    fn print_hashes(&self) {
        for block in &self.chain {
            println!("Block {}: Hash: {}", block.index, block.hash);
        }
    }
}

fn hash_image(image_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(image_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);
    Ok(format!("{:x}", hasher.finalize()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Créer une nouvelle blockchain
    let mut blockchain = Blockchain::new();

    // Client loop to add transactions
    loop {
        // Lire les informations de l'utilisateur
        println!("Enter first name:");
        let first_name: String = read!("{}\n");

        println!("Enter last name:");
        let last_name: String = read!("{}\n");

        println!("Enter class number:");
        let class_number: String = read!("{}\n");

        println!("Enter the path to your image:");
        let image_path: String = read!("{}\n");

        // Vérifier si les entrées ne sont pas vides
        if first_name.trim().is_empty() || last_name.trim().is_empty() || class_number.trim().is_empty() || image_path.trim().is_empty() {
            println!("All fields are required. Please try again.");
            continue;
        }

        // Calculer le hachage de l'image
        let image_hash = match hash_image(&image_path.trim()) {
            Ok(hash) => hash,
            Err(e) => {
                println!("Error hashing image: {}", e);
                continue;
            }
        };

        // Créer une transaction NFT avec les informations fournies
        let txn = NFTTransaction::new(first_name.trim().to_string(), last_name.trim().to_string(), class_number.trim().to_string(), image_hash);

        // Ajouter la transaction à la blockchain
        blockchain.add_block(vec![txn]);

        // Afficher la blockchain et vérifier sa validité
        println!("{:#?}", blockchain);
        println!("Is blockchain valid? {}", blockchain.is_valid());

        // Imprimer les hash des blocs
        blockchain.print_hashes();

        // Demander à l'utilisateur s'il veut ajouter une autre transaction
        println!("Do you want to add another transaction? (yes/no):");
        let answer: String = read!("{}\n");
        if answer.trim().to_lowercase() != "yes" {
            break;
        }
    }

    // Exemples supplémentaires d'utilisation de hyper et tokio pour un serveur HTTP simple
    let make_svc = make_service_fn(|_conn| {
        let blockchain = blockchain.clone();
        async {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let blockchain = blockchain.clone();
                async move {
                    match (req.method(), req.uri().path()) {
                        (&hyper::Method::GET, "/blockchain") => {
                            let body = serde_json::to_string(&blockchain).unwrap();
                            Ok::<_, hyper::Error>(Response::new(Body::from(body)))
                        },
                        _ => {
                            Ok::<_, hyper::Error>(Response::new(Body::from("Not found")))
                        },
                    }
                }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
