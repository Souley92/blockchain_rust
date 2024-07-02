use crate::block::Block;
use crate::transaction::NFTTransaction;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<NFTTransaction>,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
            pending_transactions: Vec::new(),
        };
        blockchain.chain.push(Block::new(0, Blockchain::current_time(), Vec::new(), String::new()));
        blockchain
    }

    pub fn create_transaction(&mut self, transaction: NFTTransaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self) {
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            previous_block.index + 1,
            Blockchain::current_time(),
            self.pending_transactions.clone(),
            previous_block.hash.clone(),
        );
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
        self.pending_transactions.clear();
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    fn current_time() -> u128 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        since_the_epoch.as_millis()
    }
}
