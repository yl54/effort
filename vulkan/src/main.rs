pub mod app;
pub mod window;

use app::App;

use winit::{
    EventsLoop,
    Window
};

fn main() {
    run_game();
}

fn run_game() {
    // Create an EventsLoop
    let mut events_loop = EventsLoop::new();

    // Create a window
    let window = Window::new(&events_loop);

    // Must keep in a loop to avoid just exiting immediately after 1 event
    loop {
        // Listen for events and handle event properly
        // q: what does "|event|" mean? 
        //    does it mean it moves the event into the closure? 
        //    does it mean ownership transfer?
        //    where does event come from?
        events_loop.poll_events(|event| {
            // Check the type of event that has been detected
                // event type 1

                // event type 2

                // event type 3

                // no event occurred
                    // could exit if choose to do so 
        });
    }

    // Create a triangle app instance
    let app = App{};

    // run the application
}
