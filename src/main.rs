#[macro_use]
extern crate sciter;

mod load_handler;
mod passgen;

use sciter::Value;

struct EventHandler {}

impl EventHandler {

    fn generate_password(&self, password_config_data: sciter::Value) -> String {
        let password_config = passgen::config_from_sciter_value(password_config_data);
        match password_config {
            Ok(config) => {
                println!("{:?}", config);
                String::from("Successfully generated the password!")
            },
            Err(e) => e,
        }
    }

}

impl sciter::EventHandler for EventHandler {

    dispatch_script_call! {
        fn generate_password(Value);
    }

}

fn main() {
    let _ = sciter::set_options(sciter::RuntimeOptions::DebugMode(true));

    let resources = include_bytes!("resources.rc");
    let handler = load_handler::LoadHandler::new(resources);

    let mut frame = sciter::window::Builder::main_window()
        .with_size((500, 350))
        .create();

    frame.sciter_handler(handler);
    frame.load_file("this://app/html/index.htm");

    let event_handler = EventHandler{};
    frame.event_handler(event_handler);
    frame.run_app();
}