use std::array::from_fn as arr_fn;

pub mod chunk_mesh;

const CHUNK_SIZE_16: usize = 16;

#[derive(Debug, Default)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub struct BlockState {
    pub block_type: BlockType,
}

impl BlockState {
    pub const fn new(block_type: BlockType) -> Self {
        Self { block_type }
    }
}

impl BlockState {

    pub const AIR: BlockState 
        = BlockState::new(BlockType::Air);
    
    pub const STONE: BlockState 
        = BlockState::new(BlockType::Stone);
    
    pub const DIRT: BlockState 
        = BlockState::new(BlockType::Dirt);

    pub const GRASS: BlockState 
        = BlockState::new(BlockType::Grass);
}

#[derive(Debug, Default)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub enum BlockType {
    #[default] Air,
    Dirt,
    Grass,
    Stone,
}

#[derive(Debug, Default)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub enum Biome {
    #[default] Plains,
    Desert,
    Forest,
    Jungle,
}

#[derive(Debug, Default)]
#[derive(PartialEq, Eq)]
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
        blocks: [ChunkLayer::EMPTY; CHUNK_SIZE_16],
    };
}

#[derive(Debug, Default)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
#[derive(derive_more::Deref)]
pub struct ChunkLayer(
    pub [[BlockState; CHUNK_SIZE_16]; CHUNK_SIZE_16]
);

impl ChunkLayer {
    pub fn new(inner: [[BlockState; CHUNK_SIZE_16]; CHUNK_SIZE_16]) -> Self {
        Self(inner)
    }

    /// (usize, usize) - (x, z) in chunk layer 0..16
    pub fn from_fn(mut func: impl FnMut(usize, usize) -> BlockState) -> Self {
        Self(arr_fn(|x| arr_fn(|z| func(x, z))))
    }

    pub const EMPTY: ChunkLayer = ChunkLayer(
        [const{[BlockState::AIR; CHUNK_SIZE_16]}; CHUNK_SIZE_16],
    );
}