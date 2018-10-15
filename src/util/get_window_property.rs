use x11::xlib::{
    Atom as XAtom,
    False as XFalse,
    XGetWindowProperty,
};
use std::{
    mem::uninitialized,
    os::raw::{
        c_int,
        c_uchar,
        c_ulong,
    },
    ptr::null_mut,
};
use crate::{
    Atom,
    Display,
    NotSupported,
    Window,
};

/// An export of [XGetWindowProperty].
/// Make sure to [x11::xlib::XFree] the pointer, when you're done with it.
/// 
/// An example of how to handle the response can be found in the [GetWindowPropertyResponse] docs.
pub unsafe fn get_window_property(
    display: &Display,
    window: &Window,
    property: Atom,
    expected_type: XAtom
) -> Result<GetWindowPropertyResponse, NotSupported> {
    let mut actual_type_return: XAtom = uninitialized();
    let mut actual_format_return: c_int = uninitialized();
    let mut nitems_return: c_ulong = uninitialized();
    let mut bytes_after_return: c_ulong = uninitialized();
    let mut proper_return: *mut c_uchar = null_mut();

    if XGetWindowProperty(
        display.0,
        window.0,
        property.0,
        0, 4096 / 4,
        XFalse,
        expected_type,
        &mut actual_type_return,
        &mut actual_format_return,
        &mut nitems_return,
        &mut bytes_after_return,
        &mut proper_return
    ) == 0 {
        return Ok(GetWindowPropertyResponse{
            actual_type_return,
            actual_format_return,
            nitems_return,
            bytes_after_return,
            proper_return,
        })
    }
    Err(NotSupported)
}

/// A response to [get_window_property].
/// 
/// A slice should be made from this.
/// 
/// **NOTE:** Remember to use XFree on the pointer.
/// 
/// # Example:
/// ```ignore
/// let response: GetWindowPropertyResponse = get_window_property(...);
/// if response.actual_format_return == 8 {
///     slice::from_raw_parts(response.proper_return as *const u8, response.nitems_return as usize)
///         .iter()
///         .for_each(|x| println!("{}", x));
///     XFree(response.proper_return)
/// }
/// ```
pub struct GetWindowPropertyResponse {
    /// The type of the return.
    pub actual_type_return: XAtom,
    /// The formate of the return whether it is 8, 16 or 32 bytes.
    /// If the architecture is 64-bits and the format is 32,
    /// then the return type wil be 64 bits.
    pub actual_format_return: c_int,
    /// The number of items returned in the lsit.
    pub nitems_return: c_ulong,
    /// The number of bytes that are returned.
    /// 
    /// This crate ignores this field.
    pub bytes_after_return: c_ulong,
    /// The pointer that is returned.
    pub proper_return: *const c_uchar,
}
