use std::{cell::RefCell, rc::Rc};

use egui::{Vec2, Widget};
use wgpu::{Extent3d, FilterMode};

use crate::core::{
    sf_graphics::{wgpu_backend::WgpuGraphics, world_graphics},
    world::World,
};

pub struct WorldRenderWidget {
    size: (u32, u32),
    graphics: Rc<RefCell<WgpuGraphics>>,
    render_texture: wgpu::Texture,
}

impl WorldRenderWidget {
    pub fn new(size: (u32, u32), graphics: Rc<RefCell<WgpuGraphics>>) -> Self {
        let graphics_ref = graphics.borrow();
        let texture_size = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let render_texture = graphics_ref
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("texture height"),
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: graphics_ref.surface_config.format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });

        Self {
            graphics: graphics.clone(),
            size,
            render_texture,
        }
    }

    pub fn update_size(&mut self, graphics: &WgpuGraphics, size: (u32, u32)) {
        let texture_size = wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        };

        let render_texture = graphics.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture height"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: graphics.surface_config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.render_texture = render_texture;
    }

    fn render_world_to_texture(&self, world: &World, texture_view: &wgpu::TextureView) {
        let graphics = self.graphics.borrow();

        world_graphics::render_world_to_texture(world, texture_view, &graphics);
    }

    pub fn ui(
        &self,
        ui: &mut egui::Ui,
        world: &World,
        egui_renderer: Rc<RefCell<egui_wgpu::Renderer>>,
    ) -> egui::Response {
        let texture_view = self
            .render_texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let graphics = self.graphics.borrow();

        self.render_world_to_texture(world, &texture_view);

        let egui_texture_id = egui_renderer.borrow_mut().register_native_texture(
            &graphics.device,
            &texture_view,
            FilterMode::Linear,
        );

        let sized_texture = egui::load::SizedTexture::new(
            egui_texture_id,
            Vec2::new(self.size.0 as f32, self.size.1 as f32),
        );

        let image_widget = egui::Image::new(sized_texture);

        ui.add(image_widget)
    }
}
