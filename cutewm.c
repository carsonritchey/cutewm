#include <X11/Xlib.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

#include "config.h"

void close(Display* display);
void handle_events(Display* display);
Display* init();
void init_events(Display* display); 
void on_configure_request(Display* display, const XConfigureRequestEvent e);
void on_map_request(Display* display, const XMapRequestEvent e);

bool running = true;

int main() {
	Display* display = init();

	init_events(display); 

	XSync(display, false);

	int s = DefaultScreen(display), sw = 800; 
	Window bar = XCreateSimpleWindow(display, XDefaultRootWindow(display), 0, 0, sw, 16, 1, BlackPixel(display, s), WhitePixel(display, s));
	XMapWindow(display, bar); 

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

	if(!(display = XOpenDisplay(0x0))) {
		printf("unable to open X display (looking at 0x0)\nexiting...\n");
		exit(0); 
	}

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
			printf("configurenotify event\n"); 
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
			printf("buttonpress event\n"); 
			break;
        case ButtonRelease:
			printf("buttonrelease event\n"); 
			break;
        case MotionNotify:
			//printf("motionnotify event\n"); 
			break;
		case KeyPress:
			printf("keypress event\n"); 
			break;
		case KeyRelease:
			printf("keyrelease event\n"); 
			break;

		default:
			//printf("unhandled case; ignoring\n");
			break;
	}
}

// X asks for cutewm to place and size a window
void on_configure_request(Display* display, const XConfigureRequestEvent e) {
	XWindowChanges new;

	new.x = e.x;
	new.y = e.y;
	new.width = 100;
	new.height = 100;
	new.border_width = e.border_width;
	new.sibling = e.above;
	new.stack_mode = e.detail;

	XConfigureWindow(display, e.window, e.value_mask, &new);
}

// X asks for cutewm to draw a window to the screen
void on_map_request(Display* display, const XMapRequestEvent e) {
	XMapWindow(display, e.window); 
}
