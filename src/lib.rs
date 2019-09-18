/*

Rust GUI library

*/

// TODO: Create a build file & include SDL libraries (Windows)

// TODO: IMPORTANT - Document the code according to the following:
// https://doc.rust-lang.org/rust-by-example/meta/doc.html

// For design help, see:
// https://github.com/gyscos/cursive

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
/// Widgets, Views, & Similar
pub mod view_components;
/// TODO: Refactor this out
pub mod view;
/// Font table
pub mod font;

// pub mod rust_gui {
// }