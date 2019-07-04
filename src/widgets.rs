/*

Widget functionality

Jonathan Swerdlow
7-2-19
*/

extern crate sdl2;
use sdl2::rect::Rect;

/// This is the base widget struct from which all other widgets are derived
/// # Arguments
/// 
/// * `x` - The widget's top-left x location within the window
/// * `y` - The widget's top-left y location within the window
/// * `id` - The widget's id as a string TODO: This should be a hash of the string instead for faster lookup.
pub struct WidgetBase {
    pub x: i32,
    pub y: i32,
    pub id: &'static str,
}

pub struct Button {
    pub rect: Rect,
    pub on_click: &'static Fn(), // TODO: Move this over to the trait below and allow the user to implement this??
}

trait Widget {
    /// Instatiate the widget at the given (x, y) coordinate with an optional id
    fn new(x: i32, y: i32, id: &str) {
        
    }

    /// Draw the widget to the window
    fn draw() {

    }

    // TODO: Inputs & return types (pass mouse locations, keys pressed, etc.)


    fn on_hover() {
        // Mouse hovers over widget
    }

    fn on_click() {
        // User clicks, then releases the mouse
    }

    fn on_mouse_down() {
        // User clicks (consider this for widgets such as sliders)
    }

    fn on_mouse_up() {
        // User releasese mouse (consider this for widgets such as sliders)
    }

    fn on_key_down() {
        // The user presses down a key with the current widget selected (TODO: How to select a widget? 'Highlight current'?)
    }

    fn on_key_up() {
        // The user releases a pressed key (see on_key_down)
    }
}