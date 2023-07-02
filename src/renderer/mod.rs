pub mod mesh;
pub mod shader;
pub mod camera;

pub use mesh::*;
pub use shader::*;
pub use camera::*;

use winit::{
    window::Window,
    dpi::PhysicalSize
};

use glam::*;

use crate::world::Chunk;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no valid GPU was found; fatal")]
    NoAdapterFound
}

/// The [Renderer] is responsible for managing the GPU, and rendering to the window.
pub struct Renderer {
    window: Window,
    _window_size: PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    _config: wgpu::SurfaceConfiguration,
    shader: Shader,
    pub camera: Camera,
    mesh: Mesh
}

impl Renderer {
    pub fn window(&self) -> &Window { &self.window }

    pub fn init(window: Window) -> anyhow::Result<Renderer> {
        let window_size = window.inner_size();

        log::info!("creating WGPU context");
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default()
        });
    
        log::info!("get surface from window");
        // unsafe as surface must live as long as window it was created from; fine as renderer owns both Window and Surface
        let surface = unsafe { instance.create_surface(&window) }?;
    
        log::info!("get handle to graphics card");
        let (adapter, device, queue) = pollster::block_on(async {
            // note: this will not work for all devices; may need to enumerate adapters later
            let adapter = instance.request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false
                }
            ).await.ok_or(Error::NoAdapterFound)?;

            let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::POLYGON_MODE_LINE,
                    limits: wgpu::Limits::default(),
                    label: None
                },
                None
            ).await?;

            // ugly, but need to make error type explicit
            // TODO: migrate to own error type to avoid allocation, if it becomes an issue
            Ok::<_, anyhow::Error>((adapter, device, queue))
        })?;

        let capabilities = surface.get_capabilities(&adapter);

        let surface_format = capabilities.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![]
        };

        surface.configure(&device, &config);

        let chunk = Chunk::new();

        let mesh = chunk.build_mesh(&device);

        let camera = Camera::new(
            &device,
            (0.0, 1.0, 2.0).into(),
            0.0, 0.0,
            90.0,
            1280.0 / 720.0,
            0.1, 100.0
        );

        let shader = Shader::from_source(&device, &config, &[camera.bind_group_layout()], "test_shader", include_str!("../shaders/shader.wgsl"));

        Ok(Renderer {
            window, _window_size: window_size, surface, device, queue, _config: config, shader, camera, mesh
        })
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        self.camera.update(&self.queue);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0
                        }),
                        store: true
                    }
                })],
                depth_stencil_attachment: None
            });
            
            render_pass.bind_resource(0, &self.camera);
            render_pass.use_shader(&self.shader);
            render_pass.draw_mesh(&self.mesh);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: Vec3,
    pub uv: Vec2
}

impl Vertex {
    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
            0 => Float32x3, 1 => Float32x2
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

trait DrawMesh<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh);
}

impl<'a, 'b> DrawMesh<'a> for wgpu::RenderPass<'b>
where 'a: 'b {
    fn draw_mesh(&mut self, mesh: &'a Mesh) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.draw_indexed(0..mesh.index_count, 0, 0..1);
    }
}

trait GpuResource {
    fn bind_group(&self) -> &wgpu::BindGroup;
    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;
    fn update(&self, queue: &wgpu::Queue);
}

trait BindResource<'a> {
    fn bind_resource(&mut self, index: u32, resource: &'a impl GpuResource);
}

impl<'a, 'b> BindResource<'a> for wgpu::RenderPass<'b>
where 'a: 'b {
    fn bind_resource(&mut self, index: u32, resource: &'a impl GpuResource) {
        self.set_bind_group(index, resource.bind_group(), &[]);
    }
}