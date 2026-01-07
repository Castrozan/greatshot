mod ui;
mod editor;

use std::env;

const APP_ID: &str = "io.github.syed.greatshot";

fn main() {
    use adw::prelude::*;

    let args: Vec<String> = env::args().collect();
    let capture_mode = args.iter().any(|arg| arg == "--capture" || arg == "-c");

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        ui::build_ui(app, capture_mode);
    });
    app.run();
}
