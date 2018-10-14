use x11_get_windows::Session;

fn main() {
    let mut session = Session::open()
        .expect("Could not open a new session.");
    println!("{:?}",
        session.active_window()
            .expect("Error getting the active window.")
            .get_title(&session.display)
            .expect("Error getting the title of the window."));
}
