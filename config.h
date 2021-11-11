/* stuff you can change */

// mod key (Mod1Mask = alt, Mod4Mask = super)
#define MODKEY Mod1Mask

// whether or not to draw gaps when only one window is on screen
static const int gaps_on_single = 0;
// space between windows in pixels
static const int gap_size = 10;

// whether or not cutewm will draw its bar
static int draw_bar = 1;
// whether the bar is drawn on the top or bottom of the screen (1 = top, -1 = bottom)
static const int top_bar  = 1;
// space between edge of screen and bar, in pixels (0 for no gaps)
static const int bar_vert_gap = 10;
static const int bar_hori_gap = 10;

// frame width in pixels (not to be confused with gaps)
static const int frame_width = 5;

/* stuff you probably shouldn't change */
static const int min_window_width = 32;
static const int min_window_height = min_window_width;

static const int snap_threshold = 32; 
