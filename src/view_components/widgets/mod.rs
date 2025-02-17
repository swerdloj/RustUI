pub mod button;
pub mod checkbox;
pub mod text;
pub mod textbox;
pub mod scrollbar;
pub mod image;

pub use crate::backend::system::state::GenerateView;

// Simplified widget imports
// Allows user to type `::widget::Name` or `::widget::*` rather than full namespaces
pub use button::Button;
pub use checkbox::CheckBox;
pub use text::Text;
pub use textbox::TextBox;
pub use scrollbar::ScrollBar;
pub use image::Image;


/*
    Widget Functionality
*/

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::event::Event;

use crate::backend::system::window::Window;


// ========================== WidgetState Enum ========================== //

/// Possible widget states
/// ## States
/// - `Active` - The widget is currently being clicked
/// - `Hovering` - The widget is being hovered over
/// - `Base` - The widget is in its default state
/// - `Focused` - The widget is currently focused
// TODO: See this and backend. Both need to make state names more clear
#[derive(PartialEq)]
pub enum WidgetState {
    // TODO: Should the state be called 'Active' or 'Clicking'?
    //       or should these be two different states?
    Focused,
    Active,
    Hovering,
    Base,
}


// ========================== Widget Trait ========================== //


// TODO: Consider callback types: https://oribenshir.github.io/afternoon_rusting/blog/closures

// TODO: Modify `with_text` builder methods to accept full text widgets
// This will enable the user to customize widget text without redundant methods

// Note: In this module, the generic type 'T' refers EXCLUSIVELY to user-defined state


/// The base Widget trait containing methods required for drawing & utilizing widgets
pub trait Widget<T> {
    fn rect(&self) -> Rect;
    fn id(&self) -> &'static str;

    /// Whether the widget should grab focus when clicked
    fn can_focus(&self) -> bool {
        false
    }

    /// Obtain a reference to a widget's text component for sizing/modifying
    fn text_component(&mut self) -> Option<&mut Text<T>>;

    /// Update the widget with known text dimensions  
    /// - Note that this function is called **only when text exists**  
    /// - Improper usage will therefore `panic` at `.expect()` on `None`
    fn assign_text_dimensions(&mut self, _dims: (u32, u32)) {
        panic!("Called assign_text_dimensions on a Widget that does not implement Text");
    }

    /// Modify a widget's draw origin
    fn place(&mut self, x: i32, y: i32);

    /// Trigger a callback when clicked
    fn on_click(&mut self, _state: &mut T) {
    }

    /// Render the widget to the window
    fn render(&self, window: &mut Window<T>, widget_state: WidgetState)
    where T: GenerateView<T>;

    /// Update the widget according to state & event
    // TODO: Is there anyway to avoid mutable reference here?
    //  See textbox's update fn. Persistant state would help
    fn update(&mut self, _state: &mut T, _event: &Event) {
    }

    fn cursor(&self) -> sdl2::mouse::Cursor {
        // TODO: Get this working within the backend
        sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::Hand).expect("Failed to create mouse cursor")
    }

    /// Translate the widget by the given x & y differences
    fn translate(&mut self, dx: i32, dy: i32);
    
    /// The widget's rendered width including any containers and sub-objects
    fn draw_width(&self) -> u32;

    /// The widget's rendered height including any containers and sub-objects
    fn draw_height(&self) -> u32;
}