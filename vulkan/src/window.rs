// Window
// check if window should be coupled with app or can be separate thing

use winit::{
    ControlFlow,
    Event,
    EventsLoop,
    Window as WWindow,
    WindowEvent
};
use winit::dpi::LogicalSize;

// window struct
pub struct Window {
    events_loop: EventsLoop,
    window: WWindow,
}

// impl window
impl Window {
    // function to create a window
    // prob can be ported to a separate module
    pub fn default() -> Window {
        let mut events_loop = EventsLoop::new();
        let window = match WWindow::new(&events_loop){
            Ok(w) => w,
            Err(err) => { panic!("Failed to create window: {}", err) }
        };

        Window {
            events_loop,
            window,
        }
    }

    pub fn main_loop(&mut self) {
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
            self.events_loop.poll_events(|event| {
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
    }
}


// impl drop for window
impl Drop for Window {
    // function to destroy a window
    // prob can be ported to a separate module
    fn drop(&mut self) {
        println!("dropped");
    }
}