//! x11_get_windows is a Rust crate that makes it easier to query the x11 windowing sytsem
//! to get the names of windows running on the computer.
//! 
//! [Link to online docs.](https://hirunya.github.io/docs/x11_get_windows/x11_get_windows/)
//! 
//! This is done by querying the _NET_CLIENT_LIST property for the list of windows
//! and _NET_ACTIVE_WINDOW for the current active window.
//! This seems to be a part of what is known as the
//! ["Extended Window Manager Hints"/"NetWM" standard.](https://en.wikipedia.org/wiki/Extended_Window_Manager_Hints)
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
//! 
//! # Examples
//! Getting a list of all the window titles on the screen.
//! ```ignore
//! let mut session = Session::open()
//!     .expect("Error opening a new session.");
//! session
//!     .get_windows()
//!     .expect("Could not get a list of windows.")
//!     .iter()
//!     .filter_map(|x| x.get_title(&session.display).ok())
//!     .for_each(|x| println!("{:?}", x.as_ref()))
//! // This might produce:
//! //// "Window Title 1"
//! //// "Window Title 2"
//! //// ""
//! //// "Window Title 3"
//! // etc.
//! ```
//! Get the currently active window, and find its title
//! ```ignore
//! let mut session = Session::open()
//!     .expect("Could not open a new session.");
//! println!("{:?}",
//!     session.active_window()
//!         .expect("Error getting the active window.")
//!         .get_title(&session.display)
//!         .expect("Error getting the title of the window."));
//! ```
//! 
//! If you are going to be using either a [Session] or a [Display] struct more than once,
//! please use the same one for each time as [x11::xlib::XOpenDisplay] is used when opening,
//! and [x11::xlib::XCloseDisplay] is used on drop.
//! 
//! Here are some possibly helpful links that I used when making this crate and might be helpful if you want to go past the small functionality of this crate:
//! * [Xlib - C Language X Interface](https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#Obtaining_and_Changing_Window_Properties)
//! * [StackOverflow X11: List top level windows](https://stackoverflow.com/questions/37359063/x11-list-top-level-windows)
//! * [RustDocs: x11::xlib](https://docs.rs/x11/2.18.1/x11/xlib/index.html)
//! 

#![feature(dbg_macro)]
#![feature(tool_lints)]
#![warn(clippy::all)]
#![warn(missing_docs)]
#![allow(clippy::cast_lossless, clippy::cast_ptr_alignment)]

mod atom;
mod display;
mod session;
mod window;
mod windows;
/// Just some helpful functions if you require more functionality than this wrapper.
pub mod util;

pub use self::{
    atom::Atom,
    display::Display,
    session::Session,
    window::Window,
    windows::Windows,
};

/// A struct which is used to represent that an error occured due to a Null pointer.
#[derive(Copy, Clone, Debug)]
pub struct Null;

/// A struct that represents an error where the ``_NET_ClIENT_LIST`` property
/// was not found in the root window.
/// 
/// This error can be caused by using Desktop Environments that does not support
/// the above convention.
/// The WMCTRL tool's source code that I used as a reference to make this crate
/// checked for another property, if the first one didn't work,
/// but as I had no need for it I didn't implement it.
/// But if there is a need for it I should have no problem implementing that as well.
/// 
/// Another possible source of this error was that the size of the item was not expected.
/// 
/// If this error happens please make an issue on the GitHub repo,
/// giving the OS; architecture; and/or desktop environment; of your computer.
#[derive(Copy, Clone, Debug)]
pub struct NotSupported;

const NET_CLIENT_LIST: &str = "_NET_CLIENT_LIST";

const NET_ACTIVE_WINDOW: &str = "_NET_ACTIVE_WINDOW";
