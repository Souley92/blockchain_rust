use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::fmt::{self, Formatter, Display};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash: previous_hash.clone(),
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let data = format!("{}{}{}{}{}", self.index, self.timestamp, self.data, self.previous_hash, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Block {{ index: {}, timestamp: {}, data: {}, previous_hash: {}, hash: {}, nonce: {} }}", 
               self.index, self.timestamp, self.data, self.previous_hash, self.hash, self.nonce)
    }
}
