use libc::{c_int, c_uint};
use x11::xlib;
use x11::xlib::*;

pub fn init_hotkeys(display: *mut Display) {
    unsafe {
        // mod + left click 
        xlib::XGrabButton(display, 1, MOD_KEY, xlib::XDefaultRootWindow(display), true as c_int, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
        // mod + right click
        xlib::XGrabButton(display, 3, MOD_KEY, xlib::XDefaultRootWindow(display), true as c_int, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
    }
}

/*  stuff you can change */

// mod key (Mod1Mask = alt, Mod4Mask = super) 
pub const MOD_KEY: c_uint = xlib::Mod1Mask;

/*

// character used to seperate workspace names
pub const WORKSPACE_DELIMITER: char = ' ';

// workspace names
pub const WORKSPACE_NAMES: &str = "1 2 3 4 5 6"; 

// whether or not to draw gaps when only one window is on screen
pub const GAPS_ON_SINGLE: bool = false;

// space between windows in pixels
pub const GAPS: usize = 1;

// whether or not cutewm will draw its custom bar
pub const DRAW_BAR: bool = true;

/* stuff you probably shouldn't change */

pub const MIN_WINDOW_WIDTH: usize  = 10;
pub const WIN_WINDOW_HEIGHT: usize = MIN_WINDOW_WIDTH;

pub const SNAP_THRESHHOLD: usize = 15; 

*/
