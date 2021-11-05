use x11::xlib;
use x11::xlib::*;
use std::mem::zeroed; 
use libc::c_int;

mod config;

fn main() {
    let display: *mut Display = unsafe { XOpenDisplay(&0x0) };

    if display.is_null() {
        println!("unable to get display\nexiting...");
        std::process::exit(1);
    }

    let mut attr: xlib::XWindowAttributes = unsafe { zeroed() };
    let mut start: xlib::XButtonEvent = unsafe { zeroed() }; 
    let mut event: xlib::XEvent = unsafe { zeroed() };
    start.subwindow = 0;

    config::init_hotkeys(display);

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
