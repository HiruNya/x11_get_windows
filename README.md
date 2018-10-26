# x11_get_windows

x11_get_windows is a Rust crate that makes it easier to query the x11 windowing sytsem
to get the names of windows running on the computer.

[Link to online docs.](https://hirunya.github.io/docs/x11_get_windows/x11_get_windows/)

This is done by querying the `_NET_CLIENT_LIST` property for the list of windows
and `_NET_ACTIVE_WINDOW` for the current active window.
This seems to be a part of what is known as the
["Extended Window Manager Hints"/"NetWM" standard.](https://en.wikipedia.org/wiki/Extended_Window_Manager_Hints)
However your Desktop Environment may not follow this convention and therefore
please make an issue if it doesn't work.
Furthermore as the only computer that I run this is on is running the KDE Plasma Desktop Environment
I am curious as to what other Desktop Environments can work with this.
So please feel free to tell me if it works on your Desktop Environment by making an
issue on this crate's Github repo.

The WMCTRL CLI tool's source code was used as a reference when making this.

Big Disclaimer: I am not used to writing and using raw C-bindings
and I don't write C/C++ in general,
so if you find a memory leak or some other problem,
**Please** make an issue on the Git repo.

## Examples
Getting a list of all the window titles on the screen.
```rust
let mut session = Session::open()
    .expect("Error opening a new session.");
session
    .get_windows()
    .expect("Could not get a list of windows.")
    .iter()
    .filter_map(|x| x.get_title(&session.display).ok())
    .for_each(|x| println!("{:?}", x.as_ref()))
// This might produce:
//// "Window Title 1"
//// "Window Title 2"
//// ""
//// "Window Title 3"
// etc.
```
Get the currently active window, and find its title
```rust
let mut session = Session::open()
    .expect("Could not open a new session.");
println!("{:?}",
    session.active_window()
        .expect("Error getting the active window.")
        .get_title(&session.display)
        .expect("Error getting the title of the window."));
```

If you are going to be using either a `Session` or a `Display` struct more than once,
please use the same one for each time as `x11::xlib::XOpenDisplay` is used when opening,
and `x11::xlib::XCloseDisplay` is used on drop.

## Links:
Here are some possibly helpful links that I used when making this crate and might be helpful if you want to go past the small functionality of this crate:
* [Xlib - C Language X Interface](https://www.x.org/releases/X11R7.7/doc/libX11/libX11/libX11.html#Obtaining_and_Changing_Window_Properties)
* [StackOverflow X11: List top level windows](https://stackoverflow.com/questions/37359063/x11-list-top-level-windows)
* [RustDocs: x11::xlib](https://docs.rs/x11/2.18.1/x11/xlib/index.html)

