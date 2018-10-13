use std::slice::Iter;
use crate::Window;

/// A Collection of Windows.
pub struct Windows(pub(crate) Vec<Window>);
impl Windows {
    /// Exposes the iter function of the Vec so that you can use iterator methods on it.
    pub fn iter(&self) -> Iter<Window> {
        self.0.iter()
    }
    /// Destroys the wrapper to give the inner vec.
    pub fn inner(self) -> Vec<Window> {
        self.0
    }
    /// Gives an immutable reference to the inner vec.
    pub fn as_vec(&self) -> &Vec<Window> {
        &self.0
    }
    /// Gives a mutable reference to the inner vec.
    pub fn as_vec_mut(&mut self) -> &mut Vec<Window> {
        &mut self.0
    }
}
