use x11::xlib::{
    Display as XDisplay,
    XOpenDisplay,
    XCloseDisplay,
};
use std::{
    ops::Drop,
    ptr::null,
};
use crate::Null;

/// The Display Struct is just a wrapper of a [*mut Display] from XLib.
/// 
/// When this struct is dropped, the reference will be dropped using [XCloseDisplay].
pub struct Display(pub(crate) *mut XDisplay);
impl Display {
    /// Opens a connection to the x11 server.
    /// 
    /// Will return an error of [Null] if the returned Display pointer is a null pointer.
    pub fn open() -> Result<Self, Null> {
        let x_display = unsafe { XOpenDisplay( null() ) };
        if x_display.is_null() {
            return Err(Null)
        }
        Ok(Display(x_display))
    }
    /// Consumes the safe wrapper and returns a pointer to the raw Display.
    /// 
    /// Use this if you want to get more out of the display that this crate cannot provide.
    pub unsafe fn into_raw(self) -> *mut XDisplay {
        self.0
    }
    /// Wraps a raw display pointer with a safe wrapper.
    /// 
    /// Ensure that this pointer is the only pointer as the connection is closed when this struct is dropped.
    pub unsafe fn from_raw(display: *mut XDisplay) -> Self {
        Display(display)
    }
}
impl Drop for Display {
    fn drop(&mut self) {
        unsafe { XCloseDisplay(self.0) };
    }
}
