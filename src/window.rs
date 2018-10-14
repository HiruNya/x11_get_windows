use x11::xlib::{
    Window as XWindow,
    XA_WINDOW,
    XDefaultRootWindow,
    XFree,
    XGetWMName,
    XTextProperty,
};
use std::{
    ffi::CStr,
    mem::uninitialized,
    ops::Drop,
    os::raw::c_void,
    ptr::null_mut,
    slice,
};
use crate::{
    Atom,
    Display,
    NET_ACTIVE_WINDOW,
    NotSupported,
    Null,
    util::get_window_property,
    Session,
};

/// This struct represents a window and holds the ID of that window that can be used
/// to query for its name.
#[derive(Copy, Clone, Debug)]
pub struct Window(pub XWindow);
impl Window {
    /// Gets the default root window of a display.
    /// 
    /// A wrapper around the [XDefaultRootWindow] function.
    pub fn default_root_window(display: &Display) -> Self {
        let win = unsafe { XDefaultRootWindow(display.0) };
        Window(win)
    }
    /// Gets the current active window.
    /// 
    /// This function uses a [Session] struct and will update the properties
    /// that are set to [None] but are required.
    /// This uses the display, root_window, and active_window_atom properties
    /// of the [Session] struct.
    pub fn active_window(session: &mut Session) -> Result<Self, NotSupported> {
        let Session { display, root_window, active_window_atom, .. } = session;
        let root_window = root_window.get_or_insert_with(|| Window::default_root_window(display));
        let active_window_atom = active_window_atom.get_or_insert_with(|| Atom::new(display, NET_ACTIVE_WINDOW).unwrap());
        let response = unsafe{get_window_property(display, root_window, *active_window_atom, XA_WINDOW)?};
        let window = match response.actual_format_return {
            8 => {
                unsafe{slice::from_raw_parts(response.proper_return as *const u8, response.nitems_return as usize)}
                    .first()
                    .map(|x| Window(*x as XWindow))
            },
            16 => {
                unsafe{slice::from_raw_parts(response.proper_return as *const u16, response.nitems_return as usize)}
                    .first()
                    .map(|x| Window(*x as XWindow))
            },
            32 => {
                unsafe{slice::from_raw_parts(response.proper_return as *const usize, response.nitems_return as usize)}
                    .first()
                    .map(|x| Window(*x as XWindow))
            },
            _ => { None },
        };
        unsafe{XFree(response.proper_return as *mut c_void)};
        Ok(window.ok_or(NotSupported)?)
    }
    /// Gets the title of the window.
    /// 
    /// If the window does not have a title, a null pointer may be returned.
    /// In that case the [Null] error is returned.
    /// However, I have not encountered a [Null] error yet.
    pub fn get_title(&self, display: &Display) -> Result<WindowTitle, Null> {
        let mut text_property = XTextProperty {
            value: null_mut(),
            encoding: unsafe { uninitialized() },
            format: unsafe { uninitialized() },
            nitems: unsafe { uninitialized() },
        };
        unsafe { 
            XGetWMName(
                display.0,
                self.0,
                &mut text_property,
            )
        };
        if !text_property.value.is_null() {
            let text = unsafe { CStr::from_ptr(text_property.value as *mut i8) };
            Ok(WindowTitle(text))
        } else { Err(Null) }
    }
}

#[derive(Debug)]
pub struct WindowTitle<'a>(&'a CStr);
impl<'a> AsRef<CStr> for WindowTitle<'a> {
    fn as_ref(&self) -> &CStr {
        self.0
    }
}
impl<'a> Drop for WindowTitle<'a> {
    fn drop(&mut self) {
        unsafe { XFree(self.0.as_ptr() as *mut c_void) };
    }
}
