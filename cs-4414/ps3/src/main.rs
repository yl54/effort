pub mod gash;
pub mod server;

extern crate http;
extern crate httparse;
#[macro_use]
extern crate log;
extern crate regex;

use server::handlers;
use server::webserver::Webserver;

fn main() {
    // Create a webserver.
    let mut w = Webserver::new();

    // Register some handlers.
    w.register_handler("/great".to_string(), handlers::handle_great);
    w.register_handler("/trash".to_string(), handlers::handle_trash);
    w.register_handler("/utility/date".to_string(), handlers::handle_utility_date);

    // TODO: Add a favicon

    // Listen on the webserver.
    w.listen();
}
