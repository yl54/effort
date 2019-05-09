pub mod app;
pub mod window;

use app::App;

use winit::{
    ControlFlow,
    Event,
    EventsLoop,
    Window,
    WindowEvent
};
use winit::dpi::LogicalSize;

fn main() {
    run_app();
}

fn run_app() {
    // Create an EventsLoop
    let mut events_loop = EventsLoop::new();

    // Create a window
    let window = Window::new(&events_loop);

    // Create a triangle app instance
    // q: where do i place this?
    let app = App{};

    // Must keep in a loop to avoid just exiting immediately after 1 event
    loop {
        // Listen for events and handle event properly
        // q: what does "|event|" mean? 
        //    does it mean it moves the event into the closure? 
        //    does it mean ownership transfer?
        //    where does event come from?
        // a: poll_events takes a function that references event struct
        //    figure out how poll_events works
        //    that function probably gets called and event is that thing
        events_loop.poll_events(|event| {
            // Check the type of event that has been detected
            match event {

                // Resized
                Event::WindowEvent {
                    event: WindowEvent::Resized(LogicalSize{width, height}),
                    //q: what does "missing fields `window_id`, `event`" mean?
                    // q: what is the point of ".."? 
                    ..
                } => {
                    println!("Got a resized window event.")
                },

                // q: why do some events have fields that need to be with {}, and some can be wrapped around ()?
                // KeyboardInput
                Event::WindowEvent {
                    // close requested
                    event: WindowEvent::KeyboardInput{device_id, input},
                    ..
                } => {
                    println!("Got a keyboard input window event.")
                },

                // Close Requested
                Event::WindowEvent {
                    // close requested
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("Got a close requested window event.")
                },

                // No event detected
                _ => {
                    ()
                }
            }
        });
    }

    // run the application
    // q: where does this go?
}
