use application::Application;

pub mod application;
pub mod core;
pub mod sf_log;
pub mod sf_window;

pub fn entry_point() {
    let application = Application::new();
    application.run();
}
