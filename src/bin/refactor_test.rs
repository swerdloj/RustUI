#[macro_use]
extern crate RustUI;

use RustUI::backend::system::window::Window;
// TODO: Work on imports & namespaces
use RustUI::view_components::{
    views::vstack::VStack,
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
    );

    main_window.start(view);
}