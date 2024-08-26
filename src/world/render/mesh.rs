use macroquad::prelude::*;

use super::*;

const VERT_CAP_10_000: usize = 10_000;
const IND_CAP_15_000: usize = 15_000;

pub struct Meshes {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    texture: Option<Texture2D>,
}

impl Meshes {
    pub fn new(texture: Option<Texture2D>) -> Self {
        Self {
            vertices: Vec::with_capacity(16 * 16),
            indices: Vec::with_capacity(16 * 16 * 3 / 2),
            texture,
        }
    }
}

impl Meshes {
    pub fn extend_with(
        &mut self,
        block_pos: BlockPos,
        texture: UvTexture,
        funcs: &[fn(BlockPos, UvTexture) -> [Vertex; 4]],
    ) {
        for func in funcs {
            self.indices.extend(PLANE_IND.map(|i| self.vertices.len() as u16 + i));
            self.vertices.extend(func(block_pos, texture));
        }
    }
}

#[rustfmt::skip]
const PLANE_IND: [u16; 6] = [
    0, 1, 2,
    0, 3, 2,
];

impl Meshes {

    #[rustfmt::skip]
    pub fn into_iter(self) -> impl Iterator<Item = Mesh> {
        let Meshes { mut vertices, mut indices, texture } = self;

        let mut meshes: Vec<Mesh> = Vec::with_capacity(vertices.len() / VERT_CAP_10_000);

        while vertices.len() != 0 {

            let remaining_ver = if vertices.len() > VERT_CAP_10_000 { 
                vertices.split_off(VERT_CAP_10_000)
            }
            else { vec![] };

            let remaining_ind = if vertices.len() > IND_CAP_15_000 { 
                indices.split_off(IND_CAP_15_000)
            }
            else { vec![] };

            meshes.push(Mesh {
                vertices,
                indices,
                texture: texture.clone(),
            });

            vertices = remaining_ver;
            indices = remaining_ind;
        }

        meshes.into_iter()
    }
}
