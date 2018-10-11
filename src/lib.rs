#![warn(missing_docs)]
//! x11_get_window_names is a Rust crate that makes it easier to query the x11 windowing sytsem
//! to get the names of windows running on the computer.
//! 
//! This is done by querying the _NET_CLIENT_LIST property.
//! However your Desktop Environment may not follow this convention and therefore
//! please make an issue if it doesn't work.
//! Furthermore as the only computer that I run this is on is running the KDE Plasma Desktop Environment
//! I am curious as to what other Desktop Environments can work with this.
//! So please feel free to tell me if it works on your Desktop Environment by making an
//! issue on this crate's Github repo.
//! 
//! The WMCTRL CLI tool's source code was used as a reference when making this.
//! 
//! Big Disclaimer: I am not used to writing and using raw C-bindings
//! and I don't write C/C++ in general,
//! so if you find a memory leak or some other problem,
//! **Please** make an issue on the Git repo.

mod display;
pub use self::display::Display;

/// A struct which is used to represent that an error occured due to a Null pointer.
pub struct Null;
