#include <X11/Xlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#include "config.h"

void close(Display* display);
void handle_events(Display* display);
Display* init();
void init_events(Display* display); 
void on_button_press(Display* display, const XButtonEvent);
void on_button_release(Display* display, const XButtonEvent e);
void on_configure_request(Display* display, const XConfigureRequestEvent e);
void on_key_press(Display* display, const XKeyPressedEvent e);
void on_map_request(Display* display, const XMapRequestEvent e);
void on_motion_notify(Display* display, const XButtonEvent e);
void set_cursor(Display* display, int font_index); 

bool running = true;
unsigned int sw = 0, sh = 0; // screen width and height 
int cx = -1, cy = -1;        // cursor x and y
Window* cw;                  // "cursor window", "current window" (what's being dragged or resized)
XWindowAttributes cw_attr;   // 


int main() {
	Display* display = init();

	init_events(display); 
	set_cursor(display, ptr_std); 

	//XSync(display, false);
	while(running) {
		handle_events(display);
	}
}

void close(Display* display) {
	XCloseDisplay(display);
	printf("close success\n");

	running = false;
}

Display* init() {
	Display* display;

	printf("display init\n");

	if(!(display = XOpenDisplay(0x0))) {
		printf("unable to open X display (looking at 0x0)\nexiting...\n");
		exit(0); 
	}

	Screen* screen = ScreenOfDisplay(display, 0);
	sw = screen->width;
	sh = screen->height;

	return display;
}


// tells X what events to listen for 
void init_events(Display* display) {
	unsigned long masks = 
		SubstructureRedirectMask|
		SubstructureNotifyMask|
		ButtonPressMask|
		ButtonReleaseMask|
		PointerMotionMask|
		KeyPressMask|
		KeyReleaseMask;

	XSelectInput(display, DefaultRootWindow(display), masks);
}

// main event loop
void handle_events(Display* display) {
	XEvent e;
	XNextEvent(display, &e);

	switch(e.type) {
		/*case CreateNotify: unneeded for now
			printf("createnotify event\n"); 
			break;
		case DestroyNotify:
			printf("destroynotify event\n"); 
			break;
        case ReparentNotify:
			printf("reparentnotify event\n"); 
			break;
        case MapNotify:
			printf("mapnotify event\n"); 
			break;*/
        case UnmapNotify:
			printf("unmapnotify event\n"); 
 			break;
        case ConfigureNotify:
			break;
        case MapRequest:
			printf("maprequest event\n"); 
			on_map_request(display, e.xmaprequest); 
			break;
        case ConfigureRequest:
			printf("configurerequest event\n"); 
			on_configure_request(display, e.xconfigurerequest); 
			break;
        case ButtonPress:
			on_button_press(display, e.xbutton); 
			break;
        case ButtonRelease:
			on_button_release(display, e.xbutton); 
			break;
        case MotionNotify:
			on_motion_notify(display, e.xbutton); 
			break;
		case KeyPress:
			on_key_press(display, e.xkey); 
			break;
		case KeyRelease:
			break;

		default:
			break;
	}
}

void on_button_press(Display* display, const XButtonEvent e) {
	cx = e.x_root;
	cy = e.y_root;

	if(e.subwindow != 0) 
		XGetWindowAttributes(display, e.subwindow, &cw_attr);
}

void on_button_release(Display* display, const XButtonEvent e) {
	cx = -1;
	cy = -1; 

	set_cursor(display, ptr_std); 
}

// X asks for cutewm to place and size a window
void on_configure_request(Display* display, const XConfigureRequestEvent e) {
	XWindowChanges new;

	new.x = e.x;
	new.y = e.y;
	new.width = e.width;
	new.height = e.height;
	new.border_width = e.border_width;
	new.sibling = e.above;
	new.stack_mode = e.detail;

	XConfigureWindow(display, e.window, e.value_mask, &new);
}

void on_key_press(Display* display, const XKeyPressedEvent e) {
	//printf("key code: %d\tkey state: %d\twindow: %d\n", e.keycode, e.state, e.subwindow);

	// if button press on window (and not the background (root window)) 
	if(e.subwindow != 0) {
		// closing cutewm
		if(e.state == wm_kill_mask && e.keycode == XKeysymToKeycode(display, wm_kill_key)) {
			close(display); 
			return;
		}

		// closing window
		else if(e.state == window_kill_mask && e.keycode == XKeysymToKeycode(display, window_kill_key)) {
			XDestroyWindow(display, e.subwindow); 
		}
	}
}

// X asks for cutewm to draw a window to the screen
void on_map_request(Display* display, const XMapRequestEvent e) {
	if(e.window != DefaultRootWindow(display)) {
		XMoveWindow(display, e.window, 100, 100); 
	}

	XMapWindow(display, e.window); 
}

void on_motion_notify(Display* display, const XButtonEvent e) {
	if(e.subwindow != 0) {
		int dx = e.x_root - cx, dy = e.y_root - cy;

		if(e.state == (Button1Mask|mod_key)) {
			XMoveWindow(display, e.subwindow, cw_attr.x + dx, cw_attr.y + dy); 
			set_cursor(display, ptr_moving); 
		} else if(e.state == (Button3Mask|mod_key)) {
			set_cursor(display, ptr_sizing); 

			XResizeWindow(display, e.subwindow, cw_attr.width + dx, cw_attr.height + dy); 
		}

		// focus follows mouse 
		XRaiseWindow(display, e.subwindow); 
	}
}

// sets current cursor to whatever index specified (defined in config.h) 
void set_cursor(Display* display, int font_index) {
	XDefineCursor(display, DefaultRootWindow(display), XCreateFontCursor(display, font_index)); 
}
