#[macro_use]
extern crate RustUI;

use RustUI::backend::system::window::Window;
use RustUI::widgets::widget::colors;
use RustUI::widgets::*;

use RustUI::views::{VStack, view::WidgetOrView};

fn main() {
    let mut state = 7;

    let mut main_window = Window::init("Test", &mut state);

    let view = VStack2!(
        Button::new("test")
    );

    main_window.start(view);
}