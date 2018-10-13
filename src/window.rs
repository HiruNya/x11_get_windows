use x11::xlib::{
    Window as XWindow,
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
};
use crate::{
    Display,
    Null,
};

/// This struct represents a window and holds the ID of that window that can be used
/// to query for its name.
#[derive(Copy, Clone, Debug)]
pub struct Window(pub(crate) XWindow);
impl Window {
    /// Gets the default root window of a display.
    /// 
    /// A wrapper around the [XDefaultRootWindow] function.
    pub fn default_root_window(display: &Display) -> Self {
        let win = unsafe { XDefaultRootWindow(display.0) };
        Window(win)
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
