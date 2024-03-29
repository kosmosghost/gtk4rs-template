mod window;
mod app;

use gtk::prelude::*;
use gtk::{gio};
use window::Window;

fn main() {
    // Register and include resources
    gio::resources_register_include!("gtk4rs-template.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = adw::Application::builder()
        .application_id("org.kosmosghost.gtk4rstemplate")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &adw::Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.present();
}