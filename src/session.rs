use crate::{
    Atom,
    Null,
    Display,
    Window,
};

/// This is meant to be a struct that makes it easy to use this crate.
/// 
/// This is purely for convenience.
/// 
/// Example:
/// ```rs
/// Session::open()
///     .windows()
///     .names()
///     .for_each(|title| println!("{:?}", title));
/// // Prints out the title for every window that is visible on the screen.
/// ```
pub struct Session {
    /// A display that has been opened.
    pub display: Display,
    /// The root window of the display.
    pub root_window: Option<Window>,
    client_list_atom: Option<Atom>,
}
impl Session {
    /// Opens a display.
    pub fn open() -> Result<Self, Null> {
        Ok( Self {
            display: Display::open()?,
            root_window: None,
            client_list_atom: None,
        } )
    }
    /// Creates a session from an already opened Display connection.
    /// 
    /// See [Display::open] for more information.
    pub fn from_display(display: Display) -> Self {
        Self {
            display,
            root_window: None,
            client_list_atom: None,
        }
    }
}
