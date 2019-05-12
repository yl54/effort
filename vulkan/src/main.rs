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

    // run the application
    // q: where does this go?
}
