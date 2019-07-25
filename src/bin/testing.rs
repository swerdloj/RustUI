/*

This file were serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;
extern crate sdl2;

use RustUI::backend::system::window::Window;
use RustUI::widgets::*;
use RustUI::view::{View};
use sdl2::rect::Rect;
// use sdl2::pixels::Color;

struct State {
    button_clicks: u16,
}

impl State {
    fn new() -> Self {
        State {
            button_clicks: 0,
        }
    }
}


fn main() {
    let mut app_state = State::new();

    let main_window = Window::init("Test", &mut app_state);

    // TODO: the view macro must handle default layout/padding according to the view type
    let test_vstack = VStack!(
        // Text::new("Text", "text here"),

        Button::new("Test")
            .with_on_click(Box::new(|state: &mut State| {
                state.button_clicks += 1;
                println!("Clicked the button {} times", state.button_clicks);
            }))
            .with_text("Increment"),

        Button::new("Test")
            .with_on_click(Box::new(|_| {
                println!("This doesn't increment anything");
            }))
            .with_text("This is a long message"),

        Text::new("Test", "Text Widget"),

        Button::new("Test")
            .with_on_click(Box::new(|state: &mut State| {
                example_callback(state);
            }))
            .with_text("Button"),

        Text::new("Clicks", "Counter: 0")
            .without_resize()
            .with_text_update(Box::new(|state: &State| {
                format!("Counter: {}", state.button_clicks)
            }))
    );

    // TODO: This must allow some mechanism for dynamic views
    main_window.start(test_vstack);
}

fn example_callback(_state: &mut State) {
    println!("This is a function");
}