#[macro_use]
extern crate RustUI;

use RustUI::backend::system::window::Window;
// TODO: Work on imports & namespaces
use RustUI::view_components::{
    views::{VStack, HStack, view::{View, Alignment}},
    widgets::Button,
    WidgetOrView,
    ViewComponent,
};

// use RustUI::views::{VStack, view::WidgetOrView};

fn main() {
    let mut state = 7;

    let mut main_window = Window::init("Test", &mut state);

    let view = VStack2!(
        Button::new("test")
            .with_text("Testing")
            .with_on_click(|_| {
                println!("Testing...");
            }),

        Button::new("test")
            .with_text("Testing2")
            .with_on_click(|_| {
                println!("Testing2...");
            }),
        
        Button::new("empty"),

        // Text::new("Test", "Text")

        HStack2!(
            Button::new("empty"),
            Button::new("empty"),
            Button::new("empty"),
            VStack2!(
                Button::new("empty"),
                Button::new("empty"),
                Button::new("empty"),
                Button::new("empty")
            )
        )
    )
    .alignment(Alignment::Center)
    .fixed_width(400)
    .fixed_height(400);

    // TODO: Refactor `backend.rs` to implement the new system
    main_window.start(view);
}