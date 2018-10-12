use x11::xlib::{
    Window as XWindow,
    XDefaultRootWindow,
};

use crate::Display;

/// This struct represents a window and holds the ID of that window that can be used
/// to query for its name.
pub struct Window(XWindow);
impl Window {
    /// Gets the default root window of a display.
    /// 
    /// A wrapper around the [XDefaultRootWindow] function.
    pub fn default_root_window(display: &Display) -> Self {
        let win = unsafe { XDefaultRootWindow(display.0) };
        Window(win)
    }
}
