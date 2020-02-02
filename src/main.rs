#[macro_use]
extern crate sciter;

mod load_handler;

use sciter::Value;

#[derive(Debug)]
struct PasswordConfig {
    num_words: u64,
    capitalization_enabled: bool,
    punctuation_enabled: bool,
    number_enabled: bool,
}

struct EventHandler {}

impl EventHandler {

    fn generate_password(&self, password_config_data: sciter::Value) -> bool {
        let password_config = &PasswordConfig{
            num_words: password_config_data.get_item("num-words").to_int().unwrap() as u64,
            capitalization_enabled: password_config_data.get_item("capitalization-enabled").to_bool().unwrap(),
            punctuation_enabled: password_config_data.get_item("punctuation-enabled").to_bool().unwrap(),
            number_enabled: password_config_data.get_item("number-enabled").to_bool().unwrap(),
        };
        println!("{:?}", password_config);
        false
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
