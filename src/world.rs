use std::{array::from_fn as arr_fn, ops::{Deref, DerefMut}};

use derive_more::{Deref, DerefMut};

pub mod render;

const CHUNK_SIZE_16: usize = 16;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct BlockState {
    pub block_type: BlockType,
}

impl BlockState {
    pub const fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }

    pub const fn is_empty(&self) -> bool {
        self.block_type.is_empty()
    }
}

impl BlockState {
    pub const AIR: BlockState = BlockState::new(BlockType::Air);
    pub const EMPTY: BlockState = BlockState::new(BlockType::Air);

    pub const STONE: BlockState = BlockState::new(BlockType::Stone);
    pub const DIRT: BlockState = BlockState::new(BlockType::Dirt);
    pub const GRASS: BlockState = BlockState::new(BlockType::Grass);
    pub const SAND: BlockState = BlockState::new(BlockType::Sand);
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum BlockType {
    #[default] Air,
    Dirt,
    Grass,
    Stone,
    Sand,
}

impl BlockType {
    pub const fn is_empty(&self) -> bool {
        matches!(self, BlockType::Air)
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum Biome {
    #[default] Plains,
    Desert,
    Forest,
    Jungle,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Chunk {
    pub biome: Biome,
    pub blocks: [ChunkLayer; CHUNK_SIZE_16],
}

impl Chunk {
    /// (usize, usize, usize) - pos in chunk 0..16
    pub fn from_fn(mut func: impl FnMut(usize, usize, usize) -> BlockState) -> Chunk {
        Chunk {
            biome: Biome::Plains,
            blocks: arr_fn(|y| ChunkLayer::from_fn(|x, z| func(x, y, z))),
        }
    }

    pub fn fill(&mut self, state: BlockState) {
        for y in 0..CHUNK_SIZE_16 {
            self.blocks[y].fill(state.clone());
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> &BlockState {
        self.blocks[y].get(x, z)
    }
    
    pub fn get_mut(&mut self, x: usize, y: usize, z: usize) -> &mut BlockState {
        self.blocks[y].get_mut(x, z)
    }

    pub const EMPTY: Chunk = Chunk {
        biome: Biome::Plains,
        blocks: [ChunkLayer::EMPTY; CHUNK_SIZE_16],
    };
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ChunkLayer(pub [[BlockState; CHUNK_SIZE_16]; CHUNK_SIZE_16]);

impl ChunkLayer {
    pub fn new(inner: [[BlockState; CHUNK_SIZE_16]; CHUNK_SIZE_16]) -> Self {
        Self(inner)
    }

    /// (usize, usize) - (x, z) in chunk layer 0..16
    pub fn from_fn(mut func: impl FnMut(usize, usize) -> BlockState) -> Self {
        Self(arr_fn(|x| arr_fn(|z| func(x, z))))
    }

    pub fn fill(&mut self, state: BlockState) {
        for x in 0..CHUNK_SIZE_16 {
            for z in 0..CHUNK_SIZE_16 {
                *self.get_mut(x, z) = state.clone();
            }
        }
    }

    pub fn get(&self, x: usize, z: usize) -> &BlockState {
        &self.0[x][z]
    }

    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut BlockState {
        &mut self.0[x][z]
    }

    pub fn is_empty(&self) -> bool {
        *self == ChunkLayer::EMPTY
    }

    pub const EMPTY: ChunkLayer =
        ChunkLayer([const {[BlockState::AIR; CHUNK_SIZE_16]}; CHUNK_SIZE_16]);

    
}
