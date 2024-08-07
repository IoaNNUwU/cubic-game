use super::*;

pub fn build_chunk_model(
    player_pos: Vec3, player_front: Vec3, chunk_pos: ChunkPos, 
    chunk: &Chunk, conn: &ConnectedChunks

) -> ChunkModel {
    
    let chunk_plus_connected = ChunkPlusConnected { chunk, conn };

    let chunk_pos: BlockPos = chunk_pos.into();

    let ch_pos: Vec3 = vec3(
        chunk_pos.x as f32 + CHUNK_SIZE_16 as f32 / 2., 
        chunk_pos.y as f32 + CHUNK_SIZE_16 as f32 / 2., 
        chunk_pos.z as f32 + CHUNK_SIZE_16 as f32 / 2.,
    );

    let chunk_view_vec = ch_pos - player_pos;

    let angle = player_front.angle_between(chunk_view_vec);

    let distance = player_pos.distance(ch_pos);

    if (angle > 65f32.to_radians()) && distance > 2. * CHUNK_SIZE_16 as f32 {
        return ChunkModel::EMPTY;
    }

    let mut this_chunk_model = ChunkModel::default();

    for y in 0..CHUNK_SIZE_16 {
        for x in 0..CHUNK_SIZE_16 {
            for z in 0..CHUNK_SIZE_16 {

                let block_state: &BlockState = chunk.get(x, y, z);
                if block_state.block_type == BlockType::Air { continue; }

                let conn = chunk_plus_connected.connected_blocks(x, y, z);
                let my_texture: MyTexture = my_texture(&block_state, &conn);

                let block_pos = chunk_pos + BlockPos { x: x as isize, y: y as isize, z: z as isize };

                let sides = which_3_sides_of_block_are_visible(block_pos, player_pos);

                let block_model: BlockModel = match sides {
                    ThreeSides::TopPxPz => {
                        let (top, px, pz) = (conn.top, conn.px, conn.pz);

                        match (top.is_empty(), px.is_empty(), pz.is_empty()) {
                            (true, true, true) => BlockModel::TopPxPz(my_texture.px().unwrap()),

                            (true, false, true) => BlockModel::TopPz(my_texture.px().unwrap()),
                            (true, true, false) => BlockModel::TopPx(my_texture.px().unwrap()),
                            (false, true, true) => BlockModel::PxPz(my_texture.px().unwrap()),

                            (true, false, false) => BlockModel::Top(my_texture.px().unwrap()),
                            (false, true, false) => BlockModel::Px(my_texture.px().unwrap()),
                            (false, false, true) => BlockModel::Pz(my_texture.px().unwrap()),

                            (false, false, false) => BlockModel::Empty,
                        }
                    },
                    ThreeSides::TopPxNz => {
                        let (top, px, nz) = (conn.top, conn.px, conn.nz);

                        match (top.is_empty(), px.is_empty(), nz.is_empty()) {
                            (true, true, true) => BlockModel::TopPxNz(my_texture.px().unwrap()),

                            (true, false, true) => BlockModel::TopNz(my_texture.px().unwrap()),
                            (true, true, false) => BlockModel::TopPx(my_texture.px().unwrap()),
                            (false, true, true) => BlockModel::PxNz(my_texture.px().unwrap()),

                            (true, false, false) => BlockModel::Top(my_texture.px().unwrap()),
                            (false, true, false) => BlockModel::Px(my_texture.px().unwrap()),
                            (false, false, true) => BlockModel::Nz(my_texture.px().unwrap()),

                            (false, false, false) => BlockModel::Empty,
                        }
                    },
                    ThreeSides::TopNxPz => {
                        let (top, nx, pz) = (conn.top, conn.nx, conn.pz);

                        match (top.is_empty(), nx.is_empty(), pz.is_empty()) {
                            (true, true, true) => BlockModel::TopNxPz(my_texture.px().unwrap()),

                            (true, false, true) => BlockModel::TopPz(my_texture.px().unwrap()),
                            (true, true, false) => BlockModel::TopNx(my_texture.px().unwrap()),
                            (false, true, true) => BlockModel::NxPz(my_texture.px().unwrap()),

                            (true, false, false) => BlockModel::Top(my_texture.px().unwrap()),
                            (false, true, false) => BlockModel::Nx(my_texture.px().unwrap()),
                            (false, false, true) => BlockModel::Pz(my_texture.px().unwrap()),

                            (false, false, false) => BlockModel::Empty,
                        }
                    },
                    ThreeSides::TopNxNz => {
                        let (top, nx, nz) = (conn.top, conn.nx, conn.nz);

                        match (top.is_empty(), nx.is_empty(), nz.is_empty()) {
                            (true, true, true) => BlockModel::TopNxNz(my_texture.px().unwrap()),

                            (true, false, true) => BlockModel::TopNz(my_texture.px().unwrap()),
                            (true, true, false) => BlockModel::TopNx(my_texture.px().unwrap()),
                            (false, true, true) => BlockModel::NxNz(my_texture.px().unwrap()),

                            (true, false, false) => BlockModel::Top(my_texture.px().unwrap()),
                            (false, true, false) => BlockModel::Nx(my_texture.px().unwrap()),
                            (false, false, true) => BlockModel::Nz(my_texture.px().unwrap()),

                            (false, false, false) => BlockModel::Empty,
                        }
                    },
                    ThreeSides::BottomPxPz => {
                        let (bottom, px, pz) = (conn.bottom, conn.px, conn.pz);

                        match (bottom.is_empty(), px.is_empty(), pz.is_empty()) {
                            (true, true, true) => BlockModel::BottomPxPz(my_texture.px().unwrap()),

                            (true, false, true) => BlockModel::BottomPz(my_texture.px().unwrap()),
                            (true, true, false) => BlockModel::BottomPx(my_texture.px().unwrap()),
                            (false, true, true) => BlockModel::PxPz(my_texture.px().unwrap()),

                            (true, false, false) => BlockModel::Bottom(my_texture.px().unwrap()),
                            (false, true, false) => BlockModel::Px(my_texture.px().unwrap()),
                            (false, false, true) => BlockModel::Pz(my_texture.px().unwrap()),

                            (false, false, false) => BlockModel::Empty,
                        }
                    },
                    ThreeSides::BottomPxNz => {
                        let (bottom, px, nz) = (conn.bottom, conn.px, conn.nz);

                        match (bottom.is_empty(), px.is_empty(), nz.is_empty()) {
                            (true, true, true) => BlockModel::BottomPxNz(my_texture.px().unwrap()),

                            (true, false, true) => BlockModel::BottomNz(my_texture.px().unwrap()),
                            (true, true, false) => BlockModel::BottomPx(my_texture.px().unwrap()),
                            (false, true, true) => BlockModel::PxNz(my_texture.px().unwrap()),

                            (true, false, false) => BlockModel::Bottom(my_texture.px().unwrap()),
                            (false, true, false) => BlockModel::Px(my_texture.px().unwrap()),
                            (false, false, true) => BlockModel::Nz(my_texture.px().unwrap()),

                            (false, false, false) => BlockModel::Empty,
                        }
                    },
                    ThreeSides::BottomNxPz => {
                        let (bottom, nx, pz) = (conn.bottom, conn.nx, conn.pz);

                        match (bottom.is_empty(), nx.is_empty(), pz.is_empty()) {
                            (true, true, true) => BlockModel::BottomNxPz(my_texture.px().unwrap()),

                            (true, false, true) => BlockModel::BottomPz(my_texture.px().unwrap()),
                            (true, true, false) => BlockModel::BottomNx(my_texture.px().unwrap()),
                            (false, true, true) => BlockModel::NxPz(my_texture.px().unwrap()),

                            (true, false, false) => BlockModel::Bottom(my_texture.px().unwrap()),
                            (false, true, false) => BlockModel::Nx(my_texture.px().unwrap()),
                            (false, false, true) => BlockModel::Pz(my_texture.px().unwrap()),

                            (false, false, false) => BlockModel::Empty,
                        }
                    },
                    ThreeSides::BottomNxNz => {
                        let (bottom, nx, nz) = (conn.bottom, conn.nx, conn.nz);

                        match (bottom.is_empty(), nx.is_empty(), nz.is_empty()) {
                            (true, true, true) => BlockModel::BottomNxNz(my_texture.px().unwrap()),

                            (true, false, true) => BlockModel::BottomNz(my_texture.px().unwrap()),
                            (true, true, false) => BlockModel::BottomNx(my_texture.px().unwrap()),
                            (false, true, true) => BlockModel::NxNz(my_texture.px().unwrap()),

                            (true, false, false) => BlockModel::Bottom(my_texture.px().unwrap()),
                            (false, true, false) => BlockModel::Nx(my_texture.px().unwrap()),
                            (false, false, true) => BlockModel::Nz(my_texture.px().unwrap()),

                            (false, false, false) => BlockModel::Empty,
                        }
                    },
                };

                this_chunk_model.set(x, y, z, block_model);
            }
        }
    }
    this_chunk_model
}

fn which_3_sides_of_block_are_visible(block_pos: BlockPos, player_pos: Vec3) -> ThreeSides {
    let block_pos = vec3(block_pos.x as f32, block_pos.y as f32, block_pos.z as f32);

    let (player_above, player_px, player_pz) = (
        player_pos.y > block_pos.y, player_pos.x > block_pos.x, player_pos.z > block_pos.z 
    );
    ThreeSides::from_logic(player_above, player_px, player_pz)
}

enum ThreeSides {
    TopPxPz, TopPxNz, TopNxPz, TopNxNz,
    BottomPxPz, BottomPxNz, BottomNxPz, BottomNxNz,
}

impl ThreeSides {
    fn from_logic(player_above: bool, player_px: bool, player_pz: bool) -> Self {
        match (player_above, player_px, player_pz) {
            (true, true, true) => ThreeSides::TopPxPz,
            (true, false, true) => ThreeSides::TopNxPz,
            (true, true, false) => ThreeSides::TopPxNz,
            (true, false, false) => ThreeSides::TopNxNz,
            (false, true, true) => ThreeSides::BottomPxPz,
            (false, false, true) => ThreeSides::BottomNxPz,
            (false, true, false) => ThreeSides::BottomPxNz,
            (false, false, false) => ThreeSides::BottomNxNz,
        }
    }
    pub fn top_visible(&self) -> bool {
        match self {
            ThreeSides::TopPxPz | ThreeSides::TopPxNz |
            ThreeSides::TopNxPz | ThreeSides::TopNxNz => true,
            _ => false,
        }
    }
    pub fn bottom_visible(&self) -> bool {
        !self.top_visible()
    }
    pub fn px_visible(&self) -> bool {
        match self {
            ThreeSides::TopPxPz | ThreeSides::TopPxNz | 
            ThreeSides::BottomPxPz | ThreeSides::BottomPxNz => true,
            _ => false
        }
    }
    pub fn nx_visible(&self) -> bool {
        !self.px_visible()
    }
    pub fn pz_visible(&self) -> bool {
        match self {
            ThreeSides::TopNxPz | ThreeSides::TopPxPz | 
            ThreeSides::BottomPxPz | ThreeSides::BottomNxPz => true,
            _ => false
        }
    }
    pub fn is_nz_visible(&self) -> bool {
        !self.pz_visible()
    }
}

