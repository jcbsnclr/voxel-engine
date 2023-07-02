use glam::*;
use wgpu::util::DeviceExt;

use super::GpuResource;

pub struct Camera {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,

    pub position: Vec3,
    pub pitch: f32,
    pub yaw: f32,

    pub fovy: f32,
    pub aspect: f32,
    pub znear: f32,
    pub zfar: f32
}

impl Camera {
    pub fn new(device: &wgpu::Device, position: Vec3, pitch: f32, yaw: f32, fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Camera {
        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("camera buffer"),
                mapped_at_creation: false,
                size: std::mem::size_of::<Mat4>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        );

        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None
                    }
                ],
                label: Some("camera_bind_group_layout")
            }
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("camera_bind_group"),
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding()
                    }
                ]
            }
        );

        Camera { buffer, bind_group, layout, position, pitch, yaw, fovy, aspect, znear, zfar }
    }

    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        self.yaw = (self.yaw + yaw);
        self.pitch = (self.pitch + pitch).clamp(-89.0, 89.0);
    }

    pub fn travel(&mut self, forward: bool, backward: bool, left: bool, right: bool) {
        if forward {
            self.position += self.front() * 0.01;
        }
        if backward {
            self.position -= self.front() * 0.01;
        }
        if left {
            self.position -= self.right() * 0.01;
        }
        if right {
            self.position += self.right() * 0.01;
        }
    }

    fn front(&self) -> Vec3 {
        vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        ).normalize()
    }

    fn right(&self) -> Vec3 {
        let world_up = vec3(0.0, 1.0, 0.0);
        self.front().cross(world_up).normalize()
    }

    fn up(&self) -> Vec3 {
        self.right().cross(self.front()).normalize()
    }
}

impl GpuResource for Camera {
    fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    fn update(&self, queue: &wgpu::Queue) {
        let world_up = vec3(0.0, 1.0, 0.0);

        let front = vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos()
        ).normalize();

        let right = front.cross(world_up).normalize();
        let up = right.cross(front).normalize();

        let view = glam::Mat4::look_at_rh(self.position, self.position + front, up);
        let proj = glam::Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);
        
        let matrix = proj * view;

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[matrix]));
    }
}