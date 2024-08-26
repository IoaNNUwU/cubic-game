use macroquad::models::Vertex;

use super::*;

#[path = "mesh.rs"]
mod mesh;
use mesh::*;

#[rustfmt::skip]
pub fn build_chunk_meshes(
    chunks: impl IntoIterator<Item = (ChunkPos, ChunkModel)>,
    atlas: Option<Texture2D>,
) -> impl Iterator<Item = Mesh> {
    
    let mut meshes = Meshes::new(atlas);

    for (chunk_pos, chunk_model) in chunks {

        if chunk_model.is_empty() {
            return meshes.into_iter();
        }

        let world_pos: BlockPos = chunk_pos.into();

        for y in 0..CHUNK_SIZE_16 {
            for x in 0..CHUNK_SIZE_16 {
                for z in 0..CHUNK_SIZE_16 {
                    let block_model: &BlockModel = chunk_model.get(x, y, z);

                    let block_pos = BlockPos {
                        x: x as isize + world_pos.x,
                        y: y as isize + world_pos.y,
                        z: z as isize + world_pos.z,
                    };

                    use BlockModel::*;

                    match *block_model {
                        Empty | NonCube => {}

                        Top(texture) => meshes.extend_with(block_pos, texture, &[top_vert]),
                        Bottom(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert]),
                        Px(texture) => meshes.extend_with(block_pos, texture, &[px_vert]),
                        Nx(texture) => meshes.extend_with(block_pos, texture, &[nx_vert]),
                        Pz(texture) => meshes.extend_with(block_pos, texture, &[pz_vert]),
                        Nz(texture) => meshes.extend_with(block_pos, texture, &[nz_vert]),

                        TopPx(texture) => meshes.extend_with(block_pos, texture, &[top_vert, px_vert]),
                        TopNx(texture) => meshes.extend_with(block_pos, texture, &[top_vert, nx_vert]),
                        TopPz(texture) => meshes.extend_with(block_pos, texture, &[top_vert, pz_vert]),
                        TopNz(texture) => meshes.extend_with(block_pos, texture, &[top_vert, nz_vert]),

                        BottomPx(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert, px_vert]),
                        BottomNx(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert, nx_vert]),
                        BottomPz(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert, pz_vert]),
                        BottomNz(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert, nz_vert]),

                        PxPz(texture) => meshes.extend_with(block_pos, texture, &[px_vert, pz_vert]),
                        PxNz(texture) => meshes.extend_with(block_pos, texture, &[px_vert, nz_vert]),
                        NxPz(texture) => meshes.extend_with(block_pos, texture, &[nx_vert, pz_vert]),
                        NxNz(texture) => meshes.extend_with(block_pos, texture, &[nx_vert, nz_vert]),

                        TopPxDouble(_, _) => todo!(),
                        TopNxDouble(_, _) => todo!(),
                        TopPzDouble(_, _) => todo!(),
                        TopNzDouble(_, _) => todo!(),
                        BottomPxDouble(_, _) => todo!(),
                        BottomNxDouble(_, _) => todo!(),
                        BottomPzDouble(_, _) => todo!(),
                        BottomNzDouble(_, _) => todo!(),

                        TopPxPz(texture) => meshes.extend_with(block_pos, texture, &[top_vert, px_vert, pz_vert]),
                        TopNxPz(texture) => meshes.extend_with(block_pos, texture, &[top_vert, nx_vert, pz_vert]),
                        TopPxNz(texture) => meshes.extend_with(block_pos, texture, &[top_vert, px_vert, nz_vert]),
                        TopNxNz(texture) => meshes.extend_with(block_pos, texture, &[top_vert, nx_vert, nz_vert]),

                        BottomPxPz(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert, px_vert, pz_vert]),
                        BottomNxPz(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert, nx_vert, pz_vert]),
                        BottomPxNz(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert, px_vert, nz_vert]),
                        BottomNxNz(texture) => meshes.extend_with(block_pos, texture, &[bottom_vert, nx_vert, nz_vert]),

                        TopPxPzDouble(_, _) => todo!(),
                        TopPxNzDouble(_, _) => todo!(),
                        TopNxPzDouble(_, _) => todo!(),
                        TopNxNzDouble(_, _) => todo!(),

                        BottomPxPzDouble(_, _) => todo!(),
                        BottomPxNzDouble(_, _) => todo!(),
                        BottomNxPzDouble(_, _) => todo!(),
                        BottomNxNzDouble(_, _) => todo!(),
                    };
                }
            }
        }
    };
    meshes.into_iter()
}

#[rustfmt::skip]
const fn top_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 1. + y, 0. + z), texture.low_left()),
        vertex(vec3(1. + x, 1. + y, 0. + z), texture.low_right()),
        vertex(vec3(1. + x, 1. + y, 1. + z), texture.up_right()),
        vertex(vec3(0. + x, 1. + y, 1. + z), texture.low_right()),
    ]
}

#[rustfmt::skip]
const fn bottom_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 0. + y, 0. + z), texture.low_left()),
        vertex(vec3(1. + x, 0. + y, 0. + z), texture.low_right()),
        vertex(vec3(1. + x, 0. + y, 1. + z), texture.up_right()),
        vertex(vec3(0. + x, 0. + y, 1. + z), texture.low_right()),
    ]
}

#[rustfmt::skip]
const fn px_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(1. + x, 0. + y, 0. + z), texture.low_left()),
        vertex(vec3(1. + x, 0. + y, 1. + z), texture.low_right()),
        vertex(vec3(1. + x, 1. + y, 1. + z), texture.up_right()),
        vertex(vec3(1. + x, 1. + y, 0. + z), texture.up_left()),
    ]
}

#[rustfmt::skip]
const fn nx_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 0. + y, 0. + z), texture.low_right()),
        vertex(vec3(0. + x, 0. + y, 1. + z), texture.low_left()),
        vertex(vec3(0. + x, 1. + y, 1. + z), texture.up_left()),
        vertex(vec3(0. + x, 1. + y, 0. + z), texture.up_right()),
    ]
}

#[rustfmt::skip]
const fn pz_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 0. + y, 1. + z), texture.low_right()),
        vertex(vec3(1. + x, 0. + y, 1. + z), texture.low_left()),
        vertex(vec3(1. + x, 1. + y, 1. + z), texture.up_left()),
        vertex(vec3(0. + x, 1. + y, 1. + z), texture.up_right()),
    ]
}

#[rustfmt::skip]
const fn nz_vert(pos: BlockPos, texture: UvTexture) -> [Vertex; 4] {
    let BlockPos { x, y, z } = pos;
    let (x, y, z) = (x as f32, y as f32, z as f32);

    [
        vertex(vec3(0. + x, 0. + y, 0. + z), texture.low_right()),
        vertex(vec3(1. + x, 0. + y, 0. + z), texture.low_left()),
        vertex(vec3(1. + x, 1. + y, 0. + z), texture.up_left()),
        vertex(vec3(0. + x, 1. + y, 0. + z), texture.up_right()),
    ]
}
