extern crate termion;

use std::io::{stdout, Write};
use std::{thread, time};
use termion::screen::IntoAlternateScreen;

fn main() {
    {
        let mut screen = stdout().into_alternate_screen().unwrap();
        write!(screen, "Welcome to the alternate screen.\n\nPlease wait patiently until we arrive back at the main screen in a about three seconds.").unwrap();
        screen.flush().unwrap();

        thread::sleep(time::Duration::from_secs(3));
    }

    println!("Phew! We are back.");
}
