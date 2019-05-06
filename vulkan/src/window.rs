// Window
// check if window should be coupled with app or can be separate thing


// window struct
// It will be based on the winit crate
pub struct Window {
}

// impl window
impl Window {
    // function to create a window
    // prob can be ported to a separate module
    pub fn default() -> Window {
        Window {
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