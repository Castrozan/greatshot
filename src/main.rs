mod ui;
mod editor;

use std::env;

const APP_ID: &str = "io.github.syed.greatshot";

fn main() {
    use adw::prelude::*;

    // Check for GREATSHOT_CAPTURE environment variable
    // This avoids GApplication's command-line argument processing issues
    let capture_mode = env::var("GREATSHOT_CAPTURE").is_ok();

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        ui::build_ui(app, capture_mode);
    });
    app.run();
}
