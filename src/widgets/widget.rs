/*

Widget functionality

Jonathan Swerdlow
7-2-19

TODO: Consider moving event handling (backend) to be part of Widget functionality

TODO: To properly fit view components, there must be some way of getting text rect dimensions
 from a string given font size.
 Consider a ViewComponent trait with methods such as `get_width` and `get_height`
 and implement this for all widgets and views
*/

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use crate::backend::system::window::Window;

use super::text::Text;

/// Possible widget states
/// ## States
/// * `Active` - The widget is currently being clicked
/// * `Hovering` - The widget is being hovered over
/// * `Base` - The widget is in its default state
pub enum WidgetState {
    // TODO: Should the state be called 'Active' or 'Clicking'?
    //       or should these be two different states?
    Active,
    Hovering,
    Base,
}

/// Default color implementations
pub mod colors {
    use super::Color;

    /// `Color::RGB(0, 0, 0)` - Black
    pub const BLACK: Color = Color {r: 0, g: 0, b: 0, a: 0xff};
    /// `Color::RGB(50, 50, 100)` - Default background color
    pub const DARK_PURPLE: Color = Color {r: 50, g: 50, b: 100, a: 0xff};
    /// `Color::RGB(240, 240, 200)` - Default button color
    pub const MANILLA: Color = Color {r: 240, g: 240, b: 200, a: 0xff};
    /// `Color::RGB(255, 255, 255)` - White
    pub const WHITE: Color = Color {r: 255, g: 255, b: 255, a: 0xff};
}

/// This is the base widget struct from which all other widgets are derived
/// ## Arguments
/// * `id` - The widget's id as a string TODO: This should be a hash of the string instead for faster lookup.
struct WidgetData {
    // TODO: Ensure the id is truly unique
    id: u32, // The widget's *unique* id
    rect: Rect, // Width, height, and location
    primary_color: Color, // The widget's base color (e.g.: button base color or text color)
}

// TODO: Consider callback types: https://oribenshir.github.io/afternoon_rusting/blog/closures

// TODO: Replace 'T' with 'S' for the sake of clarity?
// NOTE: In this module, the generic type 'T' refers EXCLUSIVELY to user-defined state

// TODO: Modify `with_text` builder methods to accept full text widgets
// This will enable the user to customize widget text without redundant methods


pub trait Widget<T> {
    fn rect(&self) -> Rect;
    fn id(&self) -> u32;
    fn primary_color(&self) -> Color;
    fn secondary_color(&self) -> Color;
    fn hover_color(&self) -> Color;

    fn text_component(&mut self) -> Option<&mut Text<T>>;

    /// Update the widget with known text dimensions  
    /// - Note that this function is called **only when text exists**  
    /// - Improper usage will therefore `panic` at `.expect()` on a `None` object
    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {}

    // TODO: store id hash
    fn assign_id(&mut self, id: u32);

    fn place(&mut self, x: i32, y: i32);

    fn on_click(&mut self, state: &mut T);

    /// Render the widget to the window
    fn render(&self, window: &mut Window<T>, widget_state: WidgetState);

    /// Update the widget according to state
    fn update(&mut self, state: &T);

    /// Translate the widget by the given x & y differences
    fn translate(&mut self, dx: i32, dy: i32);
    
    /// The widget's rendered width including any containers and sub-objects
    fn draw_width(&self) -> u32;

    /// Instatiate the widget with the given id.
    /// All widget fields are filled with defaults. Builder methods may be used to adjust these fields.
    // TODO: Can I not require Self::new as part of the trait because of 'Self'?
    // fn new(id: &str) -> Self
    // where Self: Sized;

    // TODO: Inputs & return types (pass mouse locations, keys pressed, etc.)


    fn on_hover() 
    where Self: Sized {
        // Mouse hovers over widget
    }

    fn on_mouse_down() 
    where Self: Sized {
        // User clicks (consider this for widgets such as sliders)
    }

    fn on_mouse_up() 
    where Self: Sized {
        // User releasese mouse (consider this for widgets such as sliders)
    }

    fn on_key_down() 
    where Self: Sized {
        // The user presses down a key with the current widget selected
    }

    fn on_key_up() 
    where Self: Sized {
        // The user releases a pressed key (see on_key_down)
    }
}