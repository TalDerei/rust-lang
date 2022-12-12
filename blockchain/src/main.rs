use blockchainlib::*;

fn main() {
    // Genesis block
    let mut block = Block::new(0, 0, vec![0; 32], 0, "Genisis Block".to_owned(), 0x0000ffffffffffffffffffffffffffff);
    
    // Mine block
    block.mine();
    println!("Mined genesis block {:?}", &block);

    // Set previous hash
    let mut last_hash = block.hash.clone();

    // Instantiate blockchain struct
    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    // Verification checks
    println!("Verify: {}", &blockchain.verify());

    // Append non-genesis blocks 
    for i in 1..=10 {
        let mut block = Block::new(i, 0, last_hash, 0, "Another Block".to_owned(), 0x0000ffffffffffffffffffffffffffff);

        block.mine();
        println!("Mined another block {:?}", &block);

        last_hash = block.hash.clone();
        blockchain.blocks.push(block);
    }
}
