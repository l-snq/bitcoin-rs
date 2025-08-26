use crate::U256;

pub struct Blockchain {
    pub blocks: Vec<Block>,
};

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
};

pub struct BlockHeader {
    pub timestamp: u64,
    pub nonce: u64,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub target: U256,
};

pub struct Transaction;

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Block {
    pub fn new(
        header: BlockHeader,
        transactions: Vec<Transaction>
        ) -> Self {
            Block {
                header: header,
                transactions: transactions,
            }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}
