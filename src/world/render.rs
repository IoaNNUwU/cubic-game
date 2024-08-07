use std::fmt::Debug;
use std::ops::Add;

use super::*;
use macroquad::prelude::*;

use derive_more::{Deref, DerefMut};


mod build_chunk_mesh;
pub use build_chunk_mesh::build_chunk_meshes;

mod build_chunk_model;
pub use build_chunk_model::build_chunk_model;


#[derive(Default, Clone, PartialEq)]
pub struct ChunkModel(Option<[ModelLayer; CHUNK_SIZE_16]>);

impl ChunkModel {
    pub const EMPTY: ChunkModel = ChunkModel(None);

    pub fn get(&self, x: usize, y: usize, z: usize) -> &BlockModel {
        match &self.0 {
            Some(arr) => arr[y].get(x, z),
            None => &BlockModel::Empty,
        }
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, model: BlockModel) {
        match &mut self.0 {
            Some(arr) => *arr[y].get_mut(x, z) = model,
            None => {
                let mut arr = [ModelLayer::EMPTY; CHUNK_SIZE_16];
                *arr[y].get_mut(x, z) = model;

                self.0 = Some(arr);
            },
        }

    }
    pub fn is_empty(&self) -> bool {
        match &self.0 {
            Some(arr) => arr.iter().all(|layer| !layer.is_empty()),
            None => true,
        }
    }
}


#[derive(Default, Clone, PartialEq)]
pub struct ModelLayer(pub [[BlockModel; CHUNK_SIZE_16]; CHUNK_SIZE_16]);

impl ModelLayer {
    pub const EMPTY: ModelLayer = ModelLayer(
        [const{[const{BlockModel::Empty}; CHUNK_SIZE_16]}; CHUNK_SIZE_16]
    );

    pub fn get(&self, x: usize, z: usize) -> &BlockModel {
        &self.0[x][z]
    }

    pub fn get_mut(&mut self, x: usize, z: usize) -> &mut BlockModel {
        &mut self.0[x][z]
    }

    pub fn is_empty(&self) -> bool {
        *self == Self::EMPTY
    }
}

struct ChunkPlusConnected<'ch, 'conn> {
    chunk: &'ch Chunk,
    conn: &'conn ConnectedChunks<'conn>,
}

impl<'ch, 'conn> ChunkPlusConnected<'ch, 'conn> {

    /// (usize, usize, usize) - (x, y, z) pos in chunk 0..16
    fn connected_blocks(&self, x: usize, y: usize, z: usize) -> ConnectedBlocks {

        let top = if y == 15 { &self.conn.top.get(x, z) } else { &self.chunk.get(x, y + 1, z) };
        let bottom = if y == 0 { &self.conn.bottom.get(x, z) } else { &self.chunk.get(x, y - 1, z) };

        let px = if x == 15 { &self.conn.px.get(x, z) } else { &self.chunk.get(x + 1, y, z) };
        let nx = if x == 0 { &self.conn.nx.get(x, z) } else { &self.chunk.get(x - 1, y, z) };
        
        let pz = if z == 15 { &self.conn.pz.get(x, z) } else { &self.chunk.get(x, y, z + 1) };
        let nz = if z == 0 { &self.conn.nz.get(x, z) } else { &self.chunk.get(x, y, z - 1) };

        ConnectedBlocks::new(&top, &bottom, &px, &nx, &pz, &nz)
    }
}

#[derive(Clone, Copy)]
pub struct ChunkPos {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl ChunkPos {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl Into<BlockPos> for ChunkPos {
    fn into(self) -> BlockPos {
        let ChunkPos { x, y, z } = self;
        BlockPos {
            x: x * CHUNK_SIZE_16 as isize,
            y: y * CHUNK_SIZE_16 as isize,
            z: z * CHUNK_SIZE_16 as isize,
        }
    }
}


#[derive(Deref, Clone, Copy, PartialEq)]
pub struct UvTexture(Vec2);

impl UvTexture {

    pub const fn from_n(n: usize) -> UvTexture {
        UvTexture::new(vec2(0., 0.01 * n as f32))
    }
    pub const fn new(inner: Vec2) -> UvTexture {
        Self(inner)
    }

    pub const fn up_left(&self) -> Vec2 {
        self.0
    }
    pub const fn up_right(&self) -> Vec2 {
        vec2(self.0.x + 1., self.0.y + 0.)
    }
    pub const fn low_left(&self) -> Vec2 {
        vec2(self.0.x + 0., self.0.y + 0.01)
    }
    pub const fn low_right(&self) -> Vec2 {
        vec2(self.0.x + 1., self.0.y + 0.01)
    }

    pub const DIRT: UvTexture = UvTexture::from_n(0);
    pub const GRASS_SIDE: UvTexture = UvTexture::from_n(1);
    pub const GRASS_TOP: UvTexture = UvTexture::from_n(2);
    pub const STONE: UvTexture = UvTexture::from_n(3);
    pub const SAND: UvTexture = UvTexture::from_n(4);
}

impl Debug for UvTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("UvTexture").field(&((self.0.y * 100.) as usize)).finish()
    }
}

impl Default for UvTexture {
    fn default() -> Self {
        Self::from_n(3)
    }
}

use macroquad::models::Vertex;

const fn vertex(pos: Vec3, uv: Vec2) -> Vertex {
    Vertex { position: pos, uv, color: WHITE }
}

#[derive(Debug, Clone, Copy)]
pub struct BlockPos {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

pub fn world_pos(x: isize, y: isize, z: isize) -> BlockPos {
    BlockPos { x, y, z }
}

impl Add for BlockPos {
    type Output = BlockPos;

    fn add(self, rhs: Self) -> Self::Output {
        let BlockPos { x, y, z } = self;
        BlockPos {
            x: x + rhs.x,
            y: y + rhs.y,
            z: z + rhs.z,
        }
    }
}

pub struct Connected<'c, T> {
    top: &'c T,
    bottom: &'c T,
    px: &'c T,
    pz: &'c T,
    nx: &'c T,
    nz: &'c T,
}

impl<'c, T> Connected<'c, T> {
    const fn all(conn: &'static T) -> Self {
        Self { top: &conn, bottom: &conn, px: &conn, pz: &conn, nx: &conn, nz: &conn }
    }
}

#[derive(Deref)]
pub struct ConnectedBlocks<'c>(Connected<'c, BlockState>);

impl<'c> ConnectedBlocks<'c> {
    pub const EMPTY: ConnectedBlocks<'c> = ConnectedBlocks(Connected::all(&BlockState::AIR));

    pub fn new(top: &'c BlockState, bottom: &'c BlockState, px: &'c BlockState, 
        pz: &'c BlockState, nx: &'c BlockState, nz: &'c BlockState) -> ConnectedBlocks<'c> {

            ConnectedBlocks(Connected { top, bottom, px, pz, nx, nz })
    }
}

#[derive(Deref)]
pub struct ConnectedChunks<'c>(Connected<'c, ChunkLayer>);

impl<'c> ConnectedChunks<'c> {
    pub const EMPTY: ConnectedChunks<'c> = ConnectedChunks(Connected::all(&ChunkLayer::EMPTY));
}

pub enum MyTexture {
    Transparent,
    AllSides(UvTexture),
    Grass {
        top: UvTexture,
        side: UvTexture,
        bottom: UvTexture,
    },
}

impl MyTexture {
    const fn top(&self) -> Option<UvTexture> {
        match self {
            MyTexture::Transparent => None,
            MyTexture::AllSides(texture) => Some(*texture),
            MyTexture::Grass { top, .. } => Some(*top),
        }
    }
    const fn bottom(&self) -> Option<UvTexture> {
        match self {
            MyTexture::Transparent => None,
            MyTexture::AllSides(texture) => Some(*texture),
            MyTexture::Grass { bottom, .. } => Some(*bottom),
        }
    }
    const fn px(&self) -> Option<UvTexture> {
        match self {
            MyTexture::Transparent => None,
            MyTexture::AllSides(texture) => Some(*texture),
            MyTexture::Grass { side, .. } => Some(*side),
        }
    }
    const fn pz(&self) -> Option<UvTexture> {
        self.px()
    }
    const fn nx(&self) -> Option<UvTexture> {
        self.px()
    }
    const fn nz(&self) -> Option<UvTexture> {
        self.px()
    }
}

// TODO: Make texture depend on connected block
// For example dirt will need to be merged with gravel etc.
const fn my_texture(bs: &BlockState, _conn: &ConnectedBlocks) -> MyTexture {
    match bs.block_type {
        BlockType::Air => MyTexture::Transparent,
        BlockType::Dirt => MyTexture::AllSides(UvTexture::DIRT),
        BlockType::Grass => MyTexture::Grass {
            top: UvTexture::GRASS_TOP,
            side: UvTexture::GRASS_SIDE,
            bottom: UvTexture::DIRT,
        },
        BlockType::Stone => MyTexture::AllSides(UvTexture::STONE),
        BlockType::Sand => MyTexture::AllSides(UvTexture::SAND),
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum BlockModel {
    
    #[default] Empty,
    NonCube,

    // 1 texture on 1 visible side of the block
    Top(UvTexture),
    Bottom(UvTexture),
    Px(UvTexture),
    Nx(UvTexture),
    Pz(UvTexture),
    Nz(UvTexture),

    // 1 texture on 2 visible sides of the block
    TopPx(UvTexture),
    TopNx(UvTexture),
    TopPz(UvTexture),
    TopNz(UvTexture),
    BottomPx(UvTexture),
    BottomNx(UvTexture),
    BottomPz(UvTexture),
    BottomNz(UvTexture),
    PxPz(UvTexture),
    PxNz(UvTexture),
    NxPz(UvTexture),
    NxNz(UvTexture),

    // 2 textures on 2 visible sides of the block
    TopPxDouble(UvTexture, UvTexture),
    TopNxDouble(UvTexture, UvTexture),
    TopPzDouble(UvTexture, UvTexture),
    TopNzDouble(UvTexture, UvTexture),
    BottomPxDouble(UvTexture, UvTexture),
    BottomNxDouble(UvTexture, UvTexture),
    BottomPzDouble(UvTexture, UvTexture),
    BottomNzDouble(UvTexture, UvTexture),

    // 1 texture on 3 visible sides of the block
    TopPxPz(UvTexture),
    TopPxNz(UvTexture),
    TopNxPz(UvTexture),
    TopNxNz(UvTexture),
    BottomPxPz(UvTexture),
    BottomPxNz(UvTexture),
    BottomNxPz(UvTexture),
    BottomNxNz(UvTexture),

    // 2 textures on 3 visible sides of the block
    TopPxPzDouble(UvTexture, UvTexture),
    TopPxNzDouble(UvTexture, UvTexture),
    TopNxPzDouble(UvTexture, UvTexture),
    TopNxNzDouble(UvTexture, UvTexture),
    BottomPxPzDouble(UvTexture, UvTexture),
    BottomPxNzDouble(UvTexture, UvTexture),
    BottomNxPzDouble(UvTexture, UvTexture),
    BottomNxNzDouble(UvTexture, UvTexture),
}

impl Debug for BlockModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty | Self::NonCube => write!(f, "[   ]"),
            Self::Top(_) => write!(f, "[+  ]"),
            Self::Bottom(_) => write!(f, "[-  ]"),
            Self::Px(_) => write!(f, "[ + ]"),
            Self::Nx(_) => write!(f, "[ - ]"),
            Self::Pz(_) => write!(f, "[  +]"),
            Self::Nz(_) => write!(f, "[  -]"),
            Self::TopPx(_) => write!(f, "[++ ]"),
            Self::TopNx(_) => write!(f, "[+- ]"),
            Self::TopPz(_) => write!(f, "[+ +]"),
            Self::TopNz(_) => write!(f, "[+ -]"),
            Self::PxPz(_) => write!(f, "[+ +]"),
            Self::PxNz(_) => write!(f, "[+ -]"),
            Self::NxPz(_) => write!(f, "[- +]"),
            Self::NxNz(_) => write!(f, "[- -]"),
            Self::BottomPx(_) => write!(f, "[-+ ]"),
            Self::BottomNx(_) => write!(f, "[-- ]"),
            Self::BottomPz(_) => write!(f, "[- +]"),
            Self::BottomNz(_) => write!(f, "[- -]"),
            Self::TopPxDouble(_, _) => write!(f, "(++ )"),
            Self::TopNxDouble(_, _) => write!(f, "(+- )"),
            Self::TopPzDouble(_, _) => write!(f, "(+ +)"),
            Self::TopNzDouble(_, _) => write!(f, "(+ -)"),
            Self::BottomPxDouble(_, _) => write!(f, "(-+ )"),
            Self::BottomNxDouble(_, _) => write!(f, "(-- )"),
            Self::BottomPzDouble(_, _) => write!(f, "(- +)"),
            Self::BottomNzDouble(_, _) => write!(f, "(- -)"),
            Self::TopPxPz(_) => write!(f, "[+++]"),
            Self::TopPxNz(_) => write!(f, "[++-]"),
            Self::TopNxPz(_) => write!(f, "[+-+]"),
            Self::TopNxNz(_) => write!(f, "[+--]"),
            Self::BottomPxPz(_) => write!(f, "[-++]"),
            Self::BottomPxNz(_) => write!(f, "[-+-]"),
            Self::BottomNxPz(_) => write!(f, "[--+]"),
            Self::BottomNxNz(_) => write!(f, "[---]"),
            Self::TopPxPzDouble(_, _) => write!(f, "(+++)"),
            Self::TopPxNzDouble(_, _) => write!(f, "(++-)"),
            Self::TopNxPzDouble(_, _) => write!(f, "(+-+)"),
            Self::TopNxNzDouble(_, _) => write!(f, "(+--)"),
            Self::BottomPxPzDouble(_, _) => write!(f, "(-++)"),
            Self::BottomPxNzDouble(_, _) => write!(f, "(-+-)"),
            Self::BottomNxPzDouble(_, _) => write!(f, "(--+)"),
            Self::BottomNxNzDouble(_, _) => write!(f, "(---)"),
        }
    }
}

impl Debug for ChunkModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..CHUNK_SIZE_16).rev() {
            for x in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    self.get(x, y, z).fmt(f)?;
                }
                write!(f, "\n")?;
            }
            write!(f, "\n\n")?;
        }
        Ok(())
    }
}