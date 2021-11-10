use libc::{c_int, c_uint};
use x11::xlib;
use x11::xlib::*;
use std::ffi::CString;

pub fn init_hotkeys(display: *mut Display) {
    unsafe {
        // mod + left click 
        xlib::XGrabButton(display, 1, MOD_KEY, xlib::XDefaultRootWindow(display), true as c_int, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
        // mod + right click
        xlib::XGrabButton(display, 3, MOD_KEY, xlib::XDefaultRootWindow(display), true as c_int, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);

        // mod + C
        let c = CString::new("c").unwrap();
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, xlib::XStringToKeysym(c.as_ptr())) as c_int, xlib::Mod1Mask, xlib::XDefaultRootWindow(display), true as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);

    }
}

/*  stuff you can change */

// mod key (Mod1Mask = alt, Mod4Mask = super) 
pub const MOD_KEY: c_uint = xlib::Mod1Mask;

/*

// character used to seperate workspace names
pub const WORKSPACE_DELIMITER: char = ' ';

// workspace names (can be multiple characters long, and of differing length)
pub const WORKSPACE_NAMES: &str = "1 2 3 4 5 6"; 

// whether or not to draw gaps when only one window is on screen
pub const GAPS_ON_SINGLE: bool = false;

// space between windows in pixels
pub const GAPS: usize = 1;

// whether or not cutewm will draw its custom bar
pub const DRAW_BAR: bool = true;

// whether or not cutewm will draw the bar at the bottom of the screen or not (top)
pub const BAR_ON_BOTTOM: bool = false; 

// space between edge of screen and bar, in pixels (0 for no gaps)
pub const BAR_VERT_GAP: i32 = 10;
pub const BAR_HORI_GAP: i32 = 10;

*/

/* stuff you probably shouldn't change */

pub const MIN_WINDOW_WIDTH: i32  = 32;
pub const MIN_WINDOW_HEIGHT: i32 = MIN_WINDOW_WIDTH;

pub const SNAP_THRESHHOLD: i32 = 30; 
