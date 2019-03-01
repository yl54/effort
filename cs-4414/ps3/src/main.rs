pub mod gash;
pub mod handlers;
pub mod server;

extern crate http;
extern crate httparse;
#[macro_use]
extern crate log;
extern crate regex;

use handlers::pool::examples;
use server::pool::webserver::Webserver;

// use server::simple::handlers;
// use server::simple::webserver::Webserver;

fn main() {
    // Create a webserver.
    let mut w = Webserver::new(1500);

    // Register some handlers.
    w.register_handler("/".to_string(), examples::handle_default, 1500);
    //w.register_handler("/great".to_string(), examples::handle_great);
    //w.register_handler("/trash".to_string(), examples::handle_trash);
    //w.register_handler("/utility/date".to_string(), //examples::handle_utility_date);

    // TODO: Add a favicon

    // Listen on the webserver.
    w.listen();
}
