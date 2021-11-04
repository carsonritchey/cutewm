use x11::xlib;
use x11::xlib::*;
use std::mem::zeroed; 
use libc::{c_int, c_uint};

mod config;

fn main() {
    let display: *mut Display = unsafe { XOpenDisplay(&0x0) };

    if display.is_null() {
        println!("unable to get display\nexiting...");
        std::process::exit(1);
    }

    // information about a given window
    let mut attr: xlib::XWindowAttributes = unsafe { zeroed() };

    // keys pressed in a given instance 
    let mut start: xlib::XButtonEvent = unsafe { zeroed() }; 

    let mut event: xlib::XEvent = unsafe { zeroed() };

    start.subwindow = 0;

    // tells xevent to listen for these key presses
    unsafe {
        // mod + left click 
        xlib::XGrabButton(display, 1, config::MOD_KEY, xlib::XDefaultRootWindow(display), true as c_int, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);

        // mod + right click
        xlib::XGrabButton(display, 3, config::MOD_KEY, xlib::XDefaultRootWindow(display), true as c_int, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
    }

    loop {
        unsafe {
            xlib::XNextEvent(display, &mut event); 

            match event.get_type() {
                xlib::ButtonPress => {
                    let xbutton: xlib::XButtonEvent = From::from(event);
                    if xbutton.subwindow != 0 {
                        xlib::XGetWindowAttributes(display, xbutton.subwindow, &mut attr); 
                        start = xbutton;
                    }
                }, 

                xlib::MotionNotify => {
                    if start.subwindow != 0 {
                        let xbutton: xlib::XButtonEvent = From::from(event);

                        // difference between where mouse started and where it is 
                        let dx: c_int = xbutton.x_root - start.x_root;
                        let dy: c_int = xbutton.y_root - start.y_root;

                        if start.button == 1 {
                            xlib::XMoveWindow(display, start.subwindow, attr.x + dx, attr.y + dy);
                        } else if start.button == 3 {
                            xlib::XResizeWindow(display, start.subwindow, (attr.width + dx) as u32, (attr.height + dy) as u32);
                        }
                    }
                }

                xlib::ButtonRelease => {
                    start.subwindow = 0;
                },

                _ => {}
            }
        }
    }
}
