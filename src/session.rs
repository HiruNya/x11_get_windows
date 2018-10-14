use x11::xlib::{
    Atom as XAtom,
    False as XFalse,
    Window as XWindow,
    XA_WINDOW,
    XFree,
    XGetWindowProperty,
};
use std::{
    mem::uninitialized,
    os::raw::{
        c_int,
        c_uchar,
        c_ulong,
        c_void,
    },
    ptr::null_mut,
    slice,
};
use crate::{
    Atom,
    Display,
    NET_CLIENT_LIST,
    NotSupported,
    Null,
    Window,
    Windows,
};

/// This is meant to be a struct that makes it easy to use this crate.
/// 
/// This is purely for convenience.
/// 
/// # Example
/// ```
/// let mut session = Session::open()
///    .expect("Error opening a new session.");
/// session
///    .get_windows()
///    .expect("Could not get a list of windows.")
///    .iter()
///    .filter_map(|x| x.get_title(&session.display).ok())
///    .for_each(|x| println!("{:?}", x.as_ref()))
/// // Prints out the title for every window that is visible on the screen.
/// ```
pub struct Session {
    /// A display that has been opened.
    pub display: Display,
    /// The root window of the display.
    pub root_window: Option<Window>,
    /// The atom that represents the client_list property.
    pub client_list_atom: Option<Atom>,
    /// The atom that represents the active_window property.
    pub active_window_atom: Option<Atom>,
}
impl Session {
    /// Opens a display.
    pub fn open() -> Result<Self, Null> {
        Ok( Self {
            display: Display::open()?,
            root_window: None,
            client_list_atom: None,
            active_window_atom: None,
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
            active_window_atom: None,
        }
    }
    /// Gets all the current windows on the screen.
    /// 
    /// This will update any values that are set to [None] if it needs to use them.
    /// 
    /// This can possible produce a [NotSupported] error.
    /// In that case, please read the documentation for that struct.
    pub fn get_windows(&mut self) -> Result<Windows, NotSupported> {
        let Session{ display, root_window, client_list_atom, .. } = self;
        let root = root_window.get_or_insert_with(|| Window::default_root_window(&display));
        let atom = client_list_atom.get_or_insert_with(|| Atom::new(&display, NET_CLIENT_LIST).unwrap());
        
        let mut return_type: XAtom = unsafe { uninitialized() };
        let mut return_format: c_int = unsafe { uninitialized() };
        let mut return_nitems: c_ulong = unsafe { uninitialized() };
        let mut return_bytes_after: c_ulong = unsafe { uninitialized() };
        let mut return_proper: *mut c_uchar = null_mut();
        if unsafe { XGetWindowProperty(
            self.display.0,
            root.0,
            atom.0,
            0, 4096 / 4,
            XFalse,
            XA_WINDOW,
            &mut return_type,
            &mut return_format,
            &mut return_nitems,
            &mut return_bytes_after,
            &mut return_proper
        ) } == 0 {
            if return_type == XA_WINDOW {
                return Ok(match return_format {
                    8 => {
                        let array = unsafe{slice::from_raw_parts(return_proper as *mut u8, return_nitems as usize)}
                            .iter()
                            .map(|x| Window(*x as XWindow))
                            .collect();
                        unsafe { XFree(return_proper as *mut c_void) };
                        Windows(array)
                    },
                    16 => {
                        let array = unsafe{slice::from_raw_parts(return_proper as *mut u16, return_nitems as usize)}
                            .iter()
                            .map(|x| Window(*x as XWindow))
                            .collect();
                        unsafe { XFree(return_proper as *mut c_void) };
                        Windows(array)
                    },
                    32 => {
                        let array = unsafe{slice::from_raw_parts(return_proper as *mut usize, return_nitems as usize)}
                            .iter()
                            .map(|x| Window(*x as XWindow))
                            .collect();
                        unsafe { XFree(return_proper as *mut c_void) };
                        Windows(array)
                    },
                    _ => return Err(NotSupported),
                })
            }
        }
        Err(NotSupported)
    }
    /// Gets the currently active window in the display.
    pub fn active_window(&mut self) -> Result<Window, NotSupported> {
        Window::active_window(self)
    }
}
