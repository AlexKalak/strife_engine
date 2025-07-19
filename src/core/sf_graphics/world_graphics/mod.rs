use std::{cell::RefCell, rc::Rc};

use crate::core::world::World;

use super::wgpu_backend::WgpuGraphics;

const VERTEX_BUFFER: [[f32; 3]; 3] = [[0.0, 0.5, 0.0], [-0.5, -0.5, 0.0], [0.5, -0.5, 0.0]];

pub fn render_world_to_texture(
    world: &World,
    texture_view: &wgpu::TextureView,
    graphics: &WgpuGraphics,
) {
    let mut encoder = graphics
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            // depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
            //     view: &self.depth_texture.view,
            //     depth_ops: Some(wgpu::Operations {
            //         load: wgpu::LoadOp::Clear(1.0),
            //         store: wgpu::StoreOp::Store,
            //     }),
            //     stencil_ops: None,
            // }),
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_vertex_buffer(0, world.vertex_buffer.slice(..));
        render_pass.set_index_buffer(world.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.set_pipeline(&world.render_pipeline);
        render_pass.draw_indexed(0..world.num_elements, 0, 0..1);
    }

    graphics.queue.submit(std::iter::once(encoder.finish()));
}
