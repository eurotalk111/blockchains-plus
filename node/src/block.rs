use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, timestamp: u64, data: String, nonce: u64) -> Self {
        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    pub fn is_valid(&self, prev_block: &Block) -> bool {
        self.previous_hash == prev_block.hash
            && self.hash == self.calculate_hash()
            && self.index == prev_block.index + 1
    }
}