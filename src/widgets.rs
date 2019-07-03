/*

Widget functionality

Jonathan Swerdlow
7-2-19
*/

/// This is the base widget struct from which all other widgets are derived
/// # Arguments
/// 
/// * `x` - The widget's top-left x location within the window
/// * `y` - The widget's top-left y location within the window
/// * `id` - The widget's id as a string TODO: This should be a hash of the string instead for faster lookup.
struct WidgetBase {
    x: i32,
    y: i32,
    id: Option<str> = None,
}

trait Widget {
    /// Instatiate the widget at the given (x, y) coordinate with an optional id
    fn new(x: i32, y: i32, id: Option<str>) -> Self {
    }

    /// Draw the widget to the window
    fn draw() -> Result {
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