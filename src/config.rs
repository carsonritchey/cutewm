use libc::c_uint;
use x11::xlib;



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

/* stuff you probably shouldn't change */

pub const MIN_WINDOW_WIDTH: usize  = 10;
pub const WIN_WINDOW_HEIGHT: usize = MIN_WINDOW_WIDTH;

*/
