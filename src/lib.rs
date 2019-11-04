/*

Rust GUI library

by Jonathan Swerdlow

*/

// TODO: Create a build file & include SDL libraries (Windows)

// TODO: IMPORTANT - Document the code according to the following:
// https://doc.rust-lang.org/rust-by-example/meta/doc.html

// For design help/ideas, see:
// https://github.com/gyscos/cursive
// &
// https://github.com/hecrj/iced

//! # Handling Data
//!   
//! All applications require state data to be passed to the backend
//! ```rust,no_run
//! struct State {...}
//! ...
//! let mut application_state = State::new();
//! let mut main_window = Window::init("Title", &mut application_state);
//!```
//! 
//! # Building Views
//! Views are built using macros and builder methods.
//! Views are created using declarative syntax.
//! ## Example
//! ```rust,no_run
//! let view = VStack!(
//!     Text::new("text_id", "default_text")
//!         .with_update(|state: &State| {
//!             format!("{}", state.field)
//!         })
//!         .color(colors::WHITE),
//! 
//!     // Nested view
//!     HStack!(
//!         Button::new("button_id1")
//!             .with_text("Button1")
//!             .on_click(|state: &mut State| {...}),
//!         Button::new("button_id2")
//!             .with_text("Button2")
//!     )
//! )
//! .fixed_width(400)
//! .align(Alignment::Center);
//! ```
//! 
//! ...

/// Library backend for handling windowing, events, etc.
pub mod backend;
/// Widgets and Views
pub mod view_components;
/// Font table
pub mod font;
/// Image functionality
pub mod images;

// ========================== Convenience Re-Exports ========================== //
pub use view_components::{views, widgets, components};
pub use view_components::colors;
// TODO: This may not be necessary
pub use view_components::{
    widgets::widget::Widget, 
    views::view::View,
    components::Component,
};
pub use view_components::views::view::Alignment;
pub use view_components::Orientation;
pub use backend::system::window::Window;
pub use backend::system::state;

// ========================== Macro Import Macro ========================== //
/// Imports items required by macros
/// - Required Traits
/// - Required Enums
/// These requirements are never exposed to the user, thus this macro is a convenience
#[macro_export]
macro_rules! macro_imports {
    () => {
        use RustUI::view_components::{ViewComponent, IntoViewComponent};
        use RustUI::view_components::views::view::View;
    };
}