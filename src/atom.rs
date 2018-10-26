use x11::xlib::{
    Atom as XAtom,
    True as XTrue,
    XInternAtom,
};
use std::ffi::{
    CString,
    NulError,
};
use crate::Display;

/// A wrapper around a [x11::xlib::Atom].
/// 
/// See the [Atom::new] function for an example on how to create one.
#[derive(Copy, Clone, Debug)]
pub struct Atom(pub XAtom);
impl Atom {
    /// An export of [XInternAtom] that turns a [CString] into a Atom.
    /// 
    /// An Error is only created if the [CString] has a null byte in it.
    /// If it does a [NulError] is returned.
    /// 
    /// # Example
    /// ```ignore
    /// Atom::new("_NET_ClIENT_LIST")
    ///     .expect("Could not create the CString");
    /// ```
    pub fn new<T: Into<Vec<u8>>>(display: &Display, text: T) -> Result<Self, NulError> {
        let text = CString::new(text)?;
        let atom = unsafe { XInternAtom(display.0, text.as_ptr(), XTrue) };
        Ok(Atom(atom))
    }
}
