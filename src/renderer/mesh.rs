use wgpu::util::DeviceExt;

use glam::*;

use super::Vertex;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u16]) -> Mesh {
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX
            }
        );

        Mesh {
            vertex_buffer, index_buffer,
            index_count: indices.len() as u32
        }
    }
}

pub struct CubeFaces(u8);

impl CubeFaces {
    pub const UP: u8    = 0b00000001;
    pub const DOWN: u8  = 0b00000010;
    pub const NORTH: u8 = 0b00000100;
    pub const SOUTH: u8 = 0b00001000;
    pub const EAST: u8  = 0b00010000;
    pub const WEST: u8  = 0b00100000;

    pub fn from_world_state(up: bool, down: bool, north: bool, south: bool, east: bool, west: bool) -> CubeFaces {
        let (up, down, north, south, east, west) = (up as u8, down as u8, north as u8, south as u8, east as u8, west as u8);

        CubeFaces(
            up * CubeFaces::UP |
            down * CubeFaces::DOWN |
            north * CubeFaces::NORTH |
            south * CubeFaces::SOUTH |
            east * CubeFaces::EAST |
            west * CubeFaces::WEST
        )
    }
}

pub struct MeshBuilder {
    vertices: Vec<Vertex>,
    indices: Vec<u16>
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder { vertices: vec![], indices: vec![] }
    }

    pub fn add_cube(&mut self, pos: glam::Vec3, faces: CubeFaces) {
        if faces.0 & CubeFaces::UP > 0 {
            let start = self.vertices.len() as u16;
            self.vertices.extend_from_slice(&[
                Vertex { position: pos + vec3(0.5, 0.5, 0.5), uv: vec2(1.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, 0.5, 0.5), uv: vec2(0.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, 0.5, -0.5), uv: vec2(0.0, 0.0)},
                Vertex { position: pos + vec3(0.5, 0.5, -0.5), uv: vec2(1.0, 0.0)},
            ]);
            self.indices.extend_from_slice(&[
                start + 2, start + 1, start + 0,
                start + 3, start + 2, start + 0
            ]);
        }
        if faces.0 & CubeFaces::DOWN > 0 {
            let start = self.vertices.len() as u16;
            self.vertices.extend_from_slice(&[
                Vertex { position: pos + vec3(0.5, -0.5, 0.5), uv: vec2(1.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, -0.5, 0.5), uv: vec2(0.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, -0.5, -0.5), uv: vec2(0.0, 0.0)},
                Vertex { position: pos + vec3(0.5, -0.5, -0.5), uv: vec2(1.0, 0.0)},
            ]);
            self.indices.extend_from_slice(&[
                start + 0, start + 1, start + 2,
                start + 0, start + 2, start + 3
            ]);
        }

        if faces.0 & CubeFaces::NORTH > 0 {
            let start = self.vertices.len() as u16;
            self.vertices.extend_from_slice(&[
                Vertex { position: pos + vec3(0.5, 0.5, 0.5), uv: vec2(1.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, 0.5, 0.5), uv: vec2(0.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, -0.5, 0.5), uv: vec2(0.0, 0.0)},
                Vertex { position: pos + vec3(0.5, -0.5, 0.5), uv: vec2(1.0, 0.0)},
            ]);
            self.indices.extend_from_slice(&[
                start + 0, start + 1, start + 2,
                start + 0, start + 2, start + 3
            ]);
        }
        if faces.0 & CubeFaces::SOUTH > 0 {
            let start = self.vertices.len() as u16;
            self.vertices.extend_from_slice(&[
                Vertex { position: pos + vec3(0.5, 0.5, -0.5), uv: vec2(1.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, 0.5, -0.5), uv: vec2(0.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, -0.5, -0.5), uv: vec2(0.0, 0.0)},
                Vertex { position: pos + vec3(0.5, -0.5, -0.5), uv: vec2(1.0, 0.0)},
            ]);
            self.indices.extend_from_slice(&[
                start + 2, start + 1, start + 0,
                start + 3, start + 2, start + 0
            ]);
        }

        if faces.0 & CubeFaces::EAST > 0 {
            let start = self.vertices.len() as u16;
            self.vertices.extend_from_slice(&[
                Vertex { position: pos + vec3(0.5, 0.5, 0.5), uv: vec2(1.0, 1.0)},
                Vertex { position: pos + vec3(0.5, 0.5, -0.5), uv: vec2(0.0, 1.0)},
                Vertex { position: pos + vec3(0.5, -0.5, -0.5), uv: vec2(0.0, 0.0)},
                Vertex { position: pos + vec3(0.5, -0.5, 0.5), uv: vec2(1.0, 0.0)},
            ]);
            self.indices.extend_from_slice(&[
                start + 2, start + 1, start + 0,
                start + 3, start + 2, start + 0
            ]);
        }
        if faces.0 & CubeFaces::WEST > 0 {
            let start = self.vertices.len() as u16;
            self.vertices.extend_from_slice(&[
                Vertex { position: pos + vec3(-0.5, 0.5, 0.5), uv: vec2(1.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, 0.5, -0.5), uv: vec2(0.0, 1.0)},
                Vertex { position: pos + vec3(-0.5, -0.5, -0.5), uv: vec2(0.0, 0.0)},
                Vertex { position: pos + vec3(-0.5, -0.5, 0.5), uv: vec2(1.0, 0.0)},
            ]);
            self.indices.extend_from_slice(&[
                start + 0, start + 1, start + 2,
                start + 0, start + 2, start + 3
            ]);
        }
    }

    pub fn build(self, device: &wgpu::Device) -> Mesh {
        Mesh::new(device, &self.vertices, &self.indices)
    }
}