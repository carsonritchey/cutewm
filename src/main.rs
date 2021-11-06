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

    unsafe { 
        xlib::XDefineCursor(display, xlib::XDefaultRootWindow(display), 0x0);
        xlib::XCreateSimpleWindow(display, xlib::XDefaultRootWindow(display), config::BAR_HORI_GAP, config::BAR_VERT_GAP, 100, 100, 0, 0, 0);
    }

    loop {
        unsafe {
            xlib::XNextEvent(display, &mut event); 

            println!("loop");

            match event.get_type() {
                xlib::KeyPress => {
                    let xkey: xlib::XKeyEvent = From::from(event);
                    if xkey.subwindow != 0 {
                        xlib::XRaiseWindow(display, xkey.subwindow); 
                    }
                },

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
                            let mut new_x = attr.width + dx;
                            if new_x < config::MIN_WINDOW_WIDTH {
                                new_x = config::MIN_WINDOW_WIDTH;
                            }
                            let mut new_y = attr.height + dy;
                            if new_y < config::MIN_WINDOW_HEIGHT {
                                new_y = config::MIN_WINDOW_HEIGHT;
                            }
                            xlib::XResizeWindow(display, start.subwindow, new_x as u32, new_y as u32);
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

    //unsafe { xlib::XCloseDisplay(display) }; 
}
