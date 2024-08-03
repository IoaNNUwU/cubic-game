use std::ops::Add;

use super::*;
use macroquad::prelude::*;

pub trait BuildBlockMesh {
    fn build_block_mesh(
        &self,
        pos: WorldPos,
        atlas: Texture2D,
        conn: &Connected<BlockState>,
    ) -> Mesh;
}

pub struct Connected<'up, 'bo, 'px, 'pz, 'nz, 'nx, T> {
    pub up: &'up T,
    pub bo: &'bo T,
    pub px: &'px T,
    pub pz: &'pz T,
    pub nz: &'nz T,
    pub nx: &'nx T,
}

impl BuildBlockMesh for BlockState {
    fn build_block_mesh(
        &self,
        pos: WorldPos,
        atlas: Texture2D,
        conn: &Connected<BlockState>,
    ) -> Mesh {
        match self.get_uv_texture() {
            TextureType::None => Mesh {
                vertices: vec![],
                indices: vec![],
                texture: Some(atlas),
            },
            TextureType::AllSides(t) => make_cube(
                CubeTextures {
                    top: t,
                    side: t,
                    bottom: t,
                },
                pos,
                atlas,
            ),
            TextureType::Grass { top, side, bottom } => {
                make_cube(CubeTextures { top, side, bottom }, pos, atlas)
            }
        }
    }
}

enum TextureType {
    None,
    AllSides(UvTexture),
    Grass {
        top: UvTexture,
        side: UvTexture,
        bottom: UvTexture,
    },
}

trait GetUvTexture {
    fn get_uv_texture(&self) -> TextureType;
}

impl GetUvTexture for BlockState {
    fn get_uv_texture(&self) -> TextureType {
        let dirt: UvTexture = UvTexture::from_n(0);
        let grass_side = UvTexture::from_n(1);
        let grass_top = UvTexture::from_n(2);
        let stone = UvTexture::from_n(3);

        match self.block_type {
            BlockType::Air => TextureType::None,
            BlockType::Dirt => TextureType::AllSides(dirt),
            BlockType::Grass => TextureType::Grass {
                top: grass_top,
                side: grass_side,
                bottom: dirt,
            },
            BlockType::Stone => TextureType::AllSides(stone),
        }
    }
}

pub trait BuildChunkMesh {
    fn build_chunk_mesh(&self, pos: ChunkPos, atlas: Texture2D, conn: &Connected<Chunk>) -> Mesh;
}

#[derive(Clone, Copy)]
pub struct ChunkPos {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Into<WorldPos> for ChunkPos {
    fn into(self) -> WorldPos {
        let ChunkPos { x, y, z } = self;
        WorldPos {
            x: x * 16,
            y: y * 16,
            z: z * 16,
        }
    }
}

impl BuildChunkMesh for Chunk {
    fn build_chunk_mesh(&self, pos: ChunkPos, atlas: Texture2D, conn: &Connected<Chunk>) -> Mesh {
        let mut vertices: Vec<Vertex> = vec![];
        let mut indices: Vec<u16> = vec![];

        let mut n = 0;

        for x in 0..CHUNK_SIZE_16 {
            for y in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    let chunk_pos: WorldPos = pos.into();

                    let block_pos = chunk_pos + world_pos(x as isize, y as isize, z as isize);

                    let connected: Connected<BlockState> = Connected {
                        up: &BlockState::AIR,
                        bo: &BlockState::AIR,
                        px: &BlockState::AIR,
                        pz: &BlockState::AIR,
                        nz: &BlockState::AIR,
                        nx: &BlockState::AIR,
                    };

                    let block_mesh =
                        self.blocks[y][x][z].build_block_mesh(block_pos, atlas.clone(), &connected);

                    if block_mesh.vertices.len() > 0 {

                        vertices.extend_from_slice(&block_mesh.vertices);

                        indices.extend(CUBE.map(|i| i + n * 24));

                        n += 1;
                    }
                }
            }
        }

        Mesh {
            vertices,
            indices,
            texture: Some(atlas.clone()),
        }
    }
}

#[derive(derive_more::Deref, Clone, Copy)]
struct UvTexture(Vec2);

impl UvTexture {
    fn from_n(n: usize) -> UvTexture {
        UvTexture::new(vec2(0., 0.01 * n as f32))
    }
    fn new(inner: Vec2) -> UvTexture {
        Self(inner)
    }

    fn up_left(&self) -> Vec2 {
        self.0
    }
    fn up_right(&self) -> Vec2 {
        self.0 + vec2(1., 0.)
    }
    fn low_left(&self) -> Vec2 {
        self.0 + vec2(0., 0.01)
    }
    fn low_right(&self) -> Vec2 {
        self.0 + vec2(1., 0.01)
    }
}

fn make_cube(textures: CubeTextures, pos: WorldPos, atlas: Texture2D) -> Mesh {
    let CubeTextures { top, side, bottom } = textures;

    let WorldPos { x, y, z } = pos;

    let (x, y, z) = (x as f32, y as f32, z as f32);

    Mesh {
        vertices: vec![
            vertex(vec3(0. + x, 0. + y, 0. + z), side.low_left()),
            vertex(vec3(1. + x, 0. + y, 0. + z), side.low_right()),
            vertex(vec3(1. + x, 1. + y, 0. + z), side.up_right()),
            vertex(vec3(0. + x, 1. + y, 0. + z), side.up_left()),
            
            vertex(vec3(0. + x, 0. + y, 1. + z), side.low_right()),
            vertex(vec3(1. + x, 0. + y, 1. + z), side.low_left()),
            vertex(vec3(1. + x, 1. + y, 1. + z), side.up_left()),
            vertex(vec3(0. + x, 1. + y, 1. + z), side.up_right()),

            vertex(vec3(0. + x, 0. + y, 0. + z), side.low_right()) ,
            vertex(vec3(0. + x, 0. + y, 1. + z), side.low_left()),
            vertex(vec3(0. + x, 1. + y, 1. + z), side.up_left()) ,
            vertex(vec3(0. + x, 1. + y, 0. + z), side.up_right()),

            vertex(vec3(1. + x, 0. + y, 0. + z), side.low_left()) ,
            vertex(vec3(1. + x, 0. + y, 1. + z), side.low_right()),
            vertex(vec3(1. + x, 1. + y, 1. + z), side.up_right()) ,
            vertex(vec3(1. + x, 1. + y, 0. + z), side.up_left()),

            vertex(vec3(0. + x, 1. + y, 0. + z), top.low_left()),
            vertex(vec3(1. + x, 1. + y, 0. + z), top.low_right()),
            vertex(vec3(1. + x, 1. + y, 1. + z), top.up_right()), 
            vertex(vec3(0. + x, 1. + y, 1. + z), top.low_right()),

            vertex(vec3(0. + x, 0. + y, 0. + z), bottom.low_left()),
            vertex(vec3(1. + x, 0. + y, 0. + z), bottom.low_right()),
            vertex(vec3(1. + x, 0. + y, 1. + z), bottom.up_right()), 
            vertex(vec3(0. + x, 0. + y, 1. + z), bottom.low_right()),
        ],
        indices: vec![
            0, 1, 2, 
            0, 3, 2, 
            
            4, 5, 6, 
            4, 7, 6,

            8, 9, 10,
            8, 11, 10,

            12, 13, 14,
            12, 15, 14,

            16, 17, 18,
            16, 19, 18,

            20, 21, 22,
            20, 23, 22,
            
        ],
        texture: Some(atlas),
    }
}
use macroquad::models::Vertex;

fn vertex(pos: Vec3, uv: Vec2) -> Vertex {
    Vertex {
        position: pos,
        uv,
        color: WHITE,
    }
}

#[derive(Clone, Copy)]
pub struct CubeTextures {
    top: UvTexture,
    side: UvTexture,
    bottom: UvTexture,
}

pub struct WorldPos {
    x: isize,
    y: isize,
    z: isize,
}

impl Add for WorldPos {
    type Output = WorldPos;

    fn add(self, rhs: Self) -> Self::Output {
        let WorldPos { x, y, z } = self;
        WorldPos {
            x: x + rhs.x,
            y: y + rhs.y,
            z: z + rhs.z,
        }
    }
}

pub fn world_pos(x: isize, y: isize, z: isize) -> WorldPos {
    WorldPos { x, y, z }
}

const CUBE: [u16; 36] = [
    0, 1, 2, 
    0, 3, 2, 
    
    4, 5, 6, 
    4, 7, 6,

    8, 9, 10,
    8, 11, 10,

    12, 13, 14,
    12, 15, 14,

    16, 17, 18,
    16, 19, 18,

    20, 21, 22,
    20, 23, 22,
    
];