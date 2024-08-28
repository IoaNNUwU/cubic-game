use macroquad::prelude::*;

use super::*;

const VERT_CAP: usize = 833 * 4;
const IND_CAP: usize = 833 * 6;

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
        let Meshes { vertices, indices, texture } = self;

        let mut meshes: Vec<Mesh> = Vec::with_capacity(vertices.len() / VERT_CAP);

        let ind = indices.chunks(IND_CAP);
        let mut vert = vertices.chunks(VERT_CAP);

        const N_IND_IN_FULL_MESH: usize = 3_332;

        for (i, meshes_ind) in ind.enumerate() {
            meshes.push(Mesh { 
                vertices: Vec::from(vert.next().unwrap()), 
                indices: Vec::from_iter(meshes_ind.iter().map(|ind| ind - (i * N_IND_IN_FULL_MESH) as u16)), 
                texture: texture.clone(),
            });
        }

        for (n, mesh) in meshes.iter().enumerate() {
            assert!(mesh.indices.len() <= IND_CAP, "Mesh #{} has {} IND", n, mesh.indices.len());
            assert!(mesh.vertices.len() <= VERT_CAP, "Mesh #{} has {} VERT", n, mesh.indices.len());
        }

        meshes.into_iter()
    }
}
