use std::{cell::RefCell, mem, rc::Rc};

use wgpu::util::DeviceExt;

use super::{
    sf_events::EventDispatcher, sf_graphics::wgpu_backend::WgpuGraphics, sf_layers::Layer,
};

pub struct WorldLayerWrapper<'a> {
    name: String,
    event_dispatcher: EventDispatcher<'a>,
}

impl<'a> Layer for WorldLayerWrapper<'a> {
    fn get_name(&mut self) -> &String {
        &self.name
    }

    fn on_attach(&mut self) {
        todo!()
    }

    fn on_detach(&mut self) {
        todo!()
    }

    fn on_update(&mut self) {}

    fn on_event(&mut self, event: &dyn super::sf_events::Eventable) {}
}

pub struct WorldLayer {}

pub struct World {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl World {
    pub fn new(graphics: Rc<RefCell<WgpuGraphics>>) -> Self {
        let graphics = graphics.borrow();
        let vertices: Vec<[f32; 3]> = vec![[0.0, 0.5, 0.0], [-0.5, -0.5, 0.0], [0.5, -0.5, 0.0]];
        let vertex_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("World Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let indices: Vec<u32> = vec![0, 1, 2];
        let index_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("World Indices buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_elements = indices.len() as u32;

        let render_pipeline_layout =
            graphics
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let shader = graphics
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(
                    include_str!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/resources/shaders/shader.wgsl"
                    ))
                    .into(),
                ),
            });

        let buffer_layout = wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                format: wgpu::VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            }],
        };

        let render_pipeline =
            graphics
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        buffers: &[buffer_layout], //2
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        //3
                        module: &shader,
                        entry_point: Some("fs_main"),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: graphics.surface_config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                        strip_index_format: None,
                        cull_mode: Some(wgpu::Face::Back),
                        front_face: wgpu::FrontFace::Ccw, // 2.
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLIP_CONTROL
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None, // 1.
                    multisample: wgpu::MultisampleState {
                        count: 1,                         // 2.
                        mask: !0,                         // 3.
                        alpha_to_coverage_enabled: false, // 4.
                    },
                    multiview: None, // 5.
                    cache: None,     // 6.
                });

        Self {
            vertex_buffer,
            index_buffer,
            num_elements,
            render_pipeline,
        }
    }
}
