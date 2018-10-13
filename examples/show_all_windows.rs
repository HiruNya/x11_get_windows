use x11_get_window_names::*;

fn main() {
    Session::open()
        .expect("Error opening a new session.")
        .get_windows()
        .expect("Could not get a list of windows.")
        .iter()
        .for_each(|x| println!("{:?}", x))
}
