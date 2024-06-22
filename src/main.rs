mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(4);

    blockchain.add_block(String::from("First Block"));
    blockchain.add_block(String::from("Second Block"));
    blockchain.add_block(String::from("Third Block"));

    for block in &blockchain.chain {
        println!("{}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
}

