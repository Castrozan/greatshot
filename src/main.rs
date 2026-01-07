mod ui;
mod editor;

use std::env;

const APP_ID: &str = "io.github.syed.greatshot";

fn main() {
    use adw::prelude::*;

    // Check for --capture flag before GApplication processes arguments
    // Remove it from args to prevent "Unknown option" error
    let mut args: Vec<String> = env::args().collect();
    let capture_mode = args.iter().any(|arg| arg == "--capture" || arg == "-c");
    
    // Remove --capture/-c from args so GApplication doesn't complain
    args.retain(|arg| arg != "--capture" && arg != "-c");
    
    // Set filtered args back (this is a workaround - GApplication will still see original args)
    // Instead, we'll use an environment variable approach
    if capture_mode {
        env::set_var("GREATSHOT_CAPTURE", "1");
    }

    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        let capture = env::var("GREATSHOT_CAPTURE").is_ok();
        ui::build_ui(app, capture);
    });
    app.run();
}
