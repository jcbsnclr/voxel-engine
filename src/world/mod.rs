use crate::renderer::{Mesh, MeshBuilder, CubeFaces};
use glam::{vec3, IVec3, ivec3};

pub struct Chunk {
    blocks: [[[bool; 16]; 16]; 16]
}

impl Chunk {
    pub fn new() -> Chunk {
        let mut chunk = Chunk { blocks: [[[false; 16]; 16]; 16] };

        chunk.blocks[11][0][10] = true;
        chunk.blocks[10][0][10] = true;
        chunk.blocks[10][0][11] = true;
        chunk.blocks[10][0][12] = true;
        chunk.blocks[10][0][13] = true;
        chunk.blocks[10][0][14] = true;
        chunk.blocks[11][0][15] = true;
        chunk.blocks[10][0][15] = true;

        chunk.blocks[13][0][11] = true;
        chunk.blocks[14][0][11] = true;
        chunk.blocks[15][0][11] = true;

        chunk.blocks[13][0][14] = true;
        chunk.blocks[14][0][14] = true;
        chunk.blocks[15][0][14] = true;

        chunk
    }

    pub fn get_faces(&self, pos: IVec3) -> CubeFaces {
        if !self.blocks[pos.x as usize][pos.y as usize][pos.z as usize] {
            return CubeFaces::from_world_state(false, false, false, false, false, false);
        }

        let up = if pos.y == 15 { true } else {
            !self.blocks[pos.x as usize][pos.y as usize + 1][pos.z as usize]
        };
        let down = if pos.y == 0 { true } else {
            !self.blocks[pos.x as usize][pos.y as usize - 1][pos.z as usize]
        };

        let north = if pos.z == 15 { true } else {
            !self.blocks[pos.x as usize][pos.y as usize][pos.z as usize + 1]
        };
        let south = if pos.z == 0 { true } else {
            !self.blocks[pos.x as usize][pos.y as usize][pos.z as usize - 1]
        };
        let east = if pos.x == 15 { true } else {
            !self.blocks[pos.x as usize + 1][pos.y as usize][pos.z as usize]
        };
        let west = if pos.x == 0 { true } else {
            !self.blocks[pos.x as usize - 1][pos.y as usize][pos.z as usize]
        };

        CubeFaces::from_world_state(up, down, north, south, east, west)
    }

    pub fn build_mesh(&self, device: &wgpu::Device) -> Mesh {
        let mut builder = MeshBuilder::new();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let faces = self.get_faces(ivec3(x as i32, y as i32, z as i32));
                    builder.add_cube(vec3(x as f32, y as f32, z as f32), faces);
                }
            }
        }

        builder.build(device)
    }
}