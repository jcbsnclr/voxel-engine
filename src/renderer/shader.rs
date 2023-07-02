use std::borrow::Cow;

use super::Vertex;

pub struct Shader {
    pipeline: wgpu::RenderPipeline
}

impl Shader {
    pub fn from_source(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, layouts: &[&wgpu::BindGroupLayout], name: &str, source: &str) -> Shader {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(name),
            source: wgpu::ShaderSource::Wgsl(Cow::from(source))
        });

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some(name),
                bind_group_layouts: layouts,
                push_constant_ranges: &[]
            }
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some(name),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::layout()]
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: config.format,
                            blend: None,
                            write_mask: wgpu::ColorWrites::ALL
                        })
                    ]
                }),

                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // polygon_mode: wgpu::PolygonMode::Line,
                    unclipped_depth: false,
                    conservative: false
                },

                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default()
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false
                },
                multiview: None
            }
        );

        Shader { pipeline }
    }
}

pub trait UseShader<'a> {
    fn use_shader(&mut self, shader: &'a Shader);
}

impl<'a, 'b> UseShader<'a> for wgpu::RenderPass<'b> 
where 'a: 'b {
    fn use_shader(&mut self, shader: &'a Shader) {
        self.set_pipeline(&shader.pipeline);
    }
}