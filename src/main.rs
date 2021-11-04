use x11::xlib::*;

mod config;

fn main() {
    let mut arg0 = 0x0 as i8;
    let display: *mut Display = unsafe { XOpenDisplay(&mut arg0) };

    if display.is_null() {
        println!("unable to get display\nexiting...");
        std::process::exit(1);
    }
}
