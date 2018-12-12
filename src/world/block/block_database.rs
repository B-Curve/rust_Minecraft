use std::collections::HashMap;
use world::block::block_type::BlockType;
use world::block::block::{Block, BlockList};

use serde;
use serde_json;
use serde_json::Error;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

const BLOCK_DEFINITION_SOURCE: &'static str = "./metadata/blocks.json";

lazy_static! {
    static ref BLOCK_DATABASE: BlockDatabase = BlockDatabase::new();
}

pub fn get() -> &'static BlockDatabase {
    &*BLOCK_DATABASE
}

pub struct BlockDatabase {
    blocks: HashMap<BlockType, Block>
}

impl BlockDatabase {
    pub fn new() -> BlockDatabase {
        let mut block_map: HashMap<BlockType, Block> = HashMap::new();
        for block in BlockDatabase::load_blocks().blocks {
            block_map.insert(block.m_type, block);
        }
        BlockDatabase { blocks: block_map }
    }

    fn load_blocks() -> BlockList {
        let file = File::open(BLOCK_DEFINITION_SOURCE).expect(&format!("Failed to open file: {}", BLOCK_DEFINITION_SOURCE));
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect(&format!("Failed to read contents of file: {}", BLOCK_DEFINITION_SOURCE));
        serde_json::from_str::<BlockList>(&contents)
            .expect(&format!("Failed to parse contents to string for file: {}", BLOCK_DEFINITION_SOURCE))
    }

    pub fn get_block(&self, id: BlockType) -> &Block {
        self.blocks.get(&id).unwrap()
    }

    pub fn unwrap_block(&self, id: Option<&BlockType>) -> &Block {
        self.blocks.get(id.unwrap_or(&BlockType::Air)).unwrap()
    }
}