use webserver::Webserver;

pub mod handlers;
pub mod utils;
pub mod webserver;

fn main() {
    // Create a webserver.
    let mut w = Webserver::new();

    // Register some handlers.
    w.register_handler("great".to_string(), handlers::handle_great);
    w.register_handler("trash".to_string(), handlers::handle_trash);

    // Listen on the webserver.
    w.listen();
}
