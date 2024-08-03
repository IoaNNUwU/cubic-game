use std::array::from_fn as arr_fn;

use const_array_init::*;

pub mod chunk_mesh;

const CHUNK_SIZE_16: usize = 16;

#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub struct BlockState {
    pub block_type: BlockType,
    // todo
}

impl BlockState {
    pub const AIR: BlockState = BlockState { 
        block_type: BlockType::Air
    };
    pub const STONE: BlockState = BlockState { 
        block_type: BlockType::Stone
    };
    pub const DIRT: BlockState = BlockState { 
        block_type: BlockType::Dirt
    };
    pub const GRASS: BlockState = BlockState { 
        block_type: BlockType::Grass
    };
}

#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub enum BlockType {
    Air  ,
    Dirt ,
    Grass,
    Stone,
}

#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub enum Biome {
    Plains,
    Desert,
    Forest,
    Jungle,
}

#[derive(Clone)]
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

    pub const EMPTY: Chunk = Chunk {
        biome: Biome::Plains,
        blocks: _EMPTY_CHUNK,
    };
}
make_const_arr!(_EMPTY_CHUNK, [ChunkLayer; 16], |_| ChunkLayer { inner: _LAYER });

#[derive(PartialEq, Eq)]
#[derive(Clone)]
#[derive(derive_more::Deref)]
pub struct ChunkLayer {
    pub inner: [[BlockState; CHUNK_SIZE_16]; CHUNK_SIZE_16]
}

impl ChunkLayer {
    pub fn new(inner: [[BlockState; CHUNK_SIZE_16]; CHUNK_SIZE_16]) -> Self {
        Self {
            inner
        }
    }

    /// (usize, usize) - (x, z) in chunk layer 0..16
    pub fn from_fn(mut func: impl FnMut(usize, usize) -> BlockState) -> Self {
        Self { 
            inner: arr_fn(|x| arr_fn(|z| func(x, z))),
        }
    }

    pub const EMPTY: ChunkLayer = ChunkLayer {
        inner: _LAYER,
    };
}
const _BLOCKS: [BlockState; 16] = const_arr!([BlockState; 16], |_| BlockState::AIR);
const _LAYER: [[BlockState; 16]; 16] = [_BLOCKS; 16];

#[test]
fn works() {
    let chunk = Chunk::from_fn(|x, y, z| {
        if x % 2 == 0 {
            BlockState::STONE
        }
        else {
            BlockState::DIRT
        }
    });
}

