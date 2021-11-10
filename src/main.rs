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

    let mut display_width: i32 = 0;
    let mut display_height: i32 = 0;
    display_width = unsafe { xlib::XDisplayWidth(display, display_width) }; 
    display_height = unsafe { xlib::XDisplayHeight(display, display_height) }; 

    config::init_hotkeys(display);

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
                            let mut new_x = attr.x + dx;
                            if new_x < config::SNAP_THRESHHOLD && new_x > -config::SNAP_THRESHHOLD {
                                new_x = 0;
                            }
                            else if new_x + attr.width > display_width - config::SNAP_THRESHHOLD && new_x + attr.width < display_width + config::SNAP_THRESHHOLD {
                                new_x = display_width - attr.width;
                            }
                            let mut new_y = attr.y + dy;
                            if new_y < config::SNAP_THRESHHOLD && new_y > -config::SNAP_THRESHHOLD {
                                new_y = 0;
                            }
                            else if new_y + attr.height > display_height - config::SNAP_THRESHHOLD && new_y + attr.height < display_height + config::SNAP_THRESHHOLD {
                                new_y = display_height - attr.height;
                            }
                            xlib::XMoveWindow(display, start.subwindow, new_x, new_y);

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
