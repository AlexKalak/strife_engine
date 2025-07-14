use core::sf_layers::Layer;

pub mod application;
pub mod core;
pub mod sf_log;
pub mod sf_window;

pub fn entry_point() {
    pollster::block_on(application::run());
}
