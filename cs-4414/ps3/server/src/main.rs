use webserver::Webserver;

pub mod webserver;

fn main() {
    // Create a webserver
    let w = Webserver::new();

    // Listen on the webserver
    w.listen();
}
