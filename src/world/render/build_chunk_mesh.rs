use super::*;

use macroquad::models::Vertex;

pub fn build_chunk_meshes(pos: ChunkPos, chunk_model: ChunkModel, atlas: Option<Texture2D>) -> Vec<Mesh> {

    let world_pos: BlockPos = pos.into();

    let mut meshes: Vec<Mesh> = vec![];

    if chunk_model.is_empty() {
        return vec![];
    }
    let mut vertices: Vec<Vertex> = Vec::with_capacity(100);
    let mut indices: Vec<u16> = Vec::with_capacity(150);

    for y in 0..CHUNK_SIZE_16 {

        for x in 0..CHUNK_SIZE_16 {
            for z in 0..CHUNK_SIZE_16 {
                let block_model: &BlockModel = chunk_model.get(x, y, z);

                use BlockModel::*;
                
                let block_pos = BlockPos { 
                    x: x as isize + world_pos.x,
                    y: y as isize + world_pos.y,
                    z: z as isize + world_pos.z,
                };

                let args = ExtendMeshArgs {
                    indices: &mut indices, vertices: &mut vertices, block_pos 
                };

                match *block_model {
                    Empty | NonCube => {},

                    Top(texture) => extend_mesh(args, texture, &[top_vert]),
                    Bottom(texture) => extend_mesh(args, texture, &[bottom_vert]),
                    Px(texture) => extend_mesh(args, texture, &[px_vert]),
                    Nx(texture) => extend_mesh(args, texture, &[nx_vert]),
                    Pz(texture) => extend_mesh(args, texture, &[pz_vert]),
                    Nz(texture) => extend_mesh(args, texture, &[nz_vert]),
                    
                    TopPx(texture) => extend_mesh(args, texture, &[top_vert, px_vert]),
                    TopNx(texture) => extend_mesh(args, texture, &[top_vert, nx_vert]),
                    TopPz(texture) => extend_mesh(args, texture, &[top_vert, pz_vert]),
                    TopNz(texture) => extend_mesh(args, texture, &[top_vert, nz_vert]),

                    BottomPx(texture) => extend_mesh(args, texture, &[bottom_vert, px_vert]),
                    BottomNx(texture) => extend_mesh(args, texture, &[bottom_vert, nx_vert]),
                    BottomPz(texture) => extend_mesh(args, texture, &[bottom_vert, pz_vert]),
                    BottomNz(texture) => extend_mesh(args, texture, &[bottom_vert, nz_vert]),

                    PxPz(texture) => extend_mesh(args, texture, &[px_vert, pz_vert]),
                    PxNz(texture) => extend_mesh(args, texture, &[px_vert, nz_vert]),
                    NxPz(texture) => extend_mesh(args, texture, &[nx_vert, pz_vert]),
                    NxNz(texture) => extend_mesh(args, texture, &[nx_vert, nz_vert]),

                    TopPxDouble(_, _) => todo!(),
                    TopNxDouble(_, _) => todo!(),
                    TopPzDouble(_, _) => todo!(),
                    TopNzDouble(_, _) => todo!(),
                    BottomPxDouble(_, _) => todo!(),
                    BottomNxDouble(_, _) => todo!(),
                    BottomPzDouble(_, _) => todo!(),
                    BottomNzDouble(_, _) => todo!(),

                    TopPxPz(texture) => extend_mesh(args, texture, &[top_vert, px_vert, pz_vert]),
                    TopNxPz(texture) => extend_mesh(args, texture, &[top_vert, nx_vert, pz_vert]),
                    TopPxNz(texture) => extend_mesh(args, texture, &[top_vert, px_vert, nz_vert]),
                    TopNxNz(texture) => extend_mesh(args, texture, &[top_vert, nx_vert, nz_vert]),

                    BottomPxPz(texture) => extend_mesh(args, texture, &[bottom_vert, px_vert, pz_vert]),
                    BottomNxPz(texture) => extend_mesh(args, texture, &[bottom_vert, nx_vert, pz_vert]),
                    BottomPxNz(texture) => extend_mesh(args, texture, &[bottom_vert, px_vert, nz_vert]),
                    BottomNxNz(texture) => extend_mesh(args, texture, &[bottom_vert, nx_vert, nz_vert]),

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
    };
    meshes.push(Mesh { vertices, indices, texture: atlas.clone() });
    meshes
}

fn extend_mesh(
    ExtendMeshArgs { indices, vertices, block_pos }: ExtendMeshArgs,
    texture: UvTexture,
    funcs: &[fn(BlockPos, UvTexture) -> [Vertex; 4]]
) {
    for func in funcs {
        indices.extend(PLANE_IND.map(|i| vertices.len() as u16 + i));
        vertices.extend(func(block_pos, texture));
    }
}

struct ExtendMeshArgs<'i, 'v> {
    indices: &'i mut Vec<u16>, vertices: &'v mut Vec<Vertex>, block_pos: BlockPos
}

const PLANE_IND: [u16; 6] = [
    0, 1, 2,
    0, 3, 2,
];

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