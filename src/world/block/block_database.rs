use std::collections::HashMap;
use world::block::block_type::BlockType;
use world::block::block::{Block, BlockList};

use serde;
use serde_json;
use serde_json::Error;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::rc::Rc;
use std::sync::Arc;

const BLOCK_DEFINITION_SOURCE: &'static str = "./metadata/blocks.json";

lazy_static! {
    static ref BLOCK_DATABASE: BlockDatabase = BlockDatabase::new();
}

pub fn get() -> &'static BlockDatabase {
    &*BLOCK_DATABASE
}

pub struct BlockDatabase {
    blocks: HashMap<BlockType, Arc<Block>>,
    natural_blocks: HashMap<BlockType, Arc<Block>>,
    unnatural_blocks: HashMap<BlockType, Arc<Block>>,
    block_height_map: HashMap<i32, Vec<BlockType>>,
}

impl BlockDatabase {
    pub fn new() -> BlockDatabase {
        let mut block_map: HashMap<BlockType, Arc<Block>> = HashMap::new();
        let mut natural_blocks: HashMap<BlockType, Arc<Block>> = HashMap::new();
        let mut unnatural_blocks: HashMap<BlockType, Arc<Block>> = HashMap::new();
        let mut block_height_map: HashMap<i32, Vec<BlockType>> = HashMap::new();
        for block in BlockDatabase::load_blocks().blocks {
            let block_arc = Arc::new(block);
            block_map.insert(block_arc.m_type, block_arc.clone());
            match block_arc.natural {
                true => natural_blocks.insert(block_arc.m_type, block_arc.clone()),
                false => unnatural_blocks.insert(block_arc.m_type, block_arc.clone())
            };
            if !block_arc.natural { continue; }
            for y in block_arc.min_height..block_arc.max_height {
                let entries = block_height_map.entry(y).or_insert(Vec::new());
                for r in 0..(100 - block_arc.rarity) {
                    entries.push(block_arc.m_type.clone())
                }
            }
        }
        BlockDatabase { blocks: block_map, natural_blocks, unnatural_blocks, block_height_map }
    }

    fn load_blocks() -> BlockList {
        let file = File::open(BLOCK_DEFINITION_SOURCE).expect(&format!("Failed to open file: {}", BLOCK_DEFINITION_SOURCE));
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect(&format!("Failed to read contents of file: {}", BLOCK_DEFINITION_SOURCE));
        serde_json::from_str::<BlockList>(&contents)
            .expect(&format!("Failed to parse contents to string for file: {}", BLOCK_DEFINITION_SOURCE))
    }

    pub fn blocks_at_height(&self, height: i32) -> &Vec<BlockType> {
        self.block_height_map.get(&height).unwrap()
    }

    pub fn natural_blocks(&self) -> &HashMap<BlockType, Arc<Block>> {
        &self.natural_blocks
    }

    pub fn unnatural_blocks(&self) -> &HashMap<BlockType, Arc<Block>> {
        &self.unnatural_blocks
    }

    pub fn get_block(&self, id: BlockType) -> Arc<Block> {
        unsafe {
            Arc::clone(self.blocks.get(&id).unwrap())
        }
    }

    pub fn unwrap_block(&self, id: Option<&BlockType>) -> Arc<Block> {
        unsafe {
            Arc::clone(self.blocks.get(id.unwrap_or(&BlockType::Air)).unwrap())
        }
    }
}