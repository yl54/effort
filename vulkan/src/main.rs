pub mod app;
pub mod app_vk;
pub mod window;

use app::App;

use winit::dpi::LogicalSize;
use winit::{ControlFlow, Event, EventsLoop, WindowEvent};

use window::Window;

fn main() {
    run_app();
}

fn run_app() {
    // Create a window
    let mut window = Window::default();

    // Create a triangle app instance
    // q: where do i place this?
    //let app = App {};

    window.main_loop();

    // run the application
    // q: where does this go?
}
