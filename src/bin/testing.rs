/*

This file were serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;
extern crate sdl2;

use RustUI::backend::system::window::Window;
use RustUI::widgets::*;
use RustUI::view::{View, SubView};

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
        Text::new("Clicks", "Counter: 0")
            .without_resize()
            .with_text_update(Box::new(|state: &State| {
                format!("Counter: {}", state.button_clicks)
            }))
            .with_color(255, 255, 255),

        Button::new("Test")
            .with_on_click(Box::new(|state: &mut State| {
                state.button_clicks += 1;
                println!("Clicked the button {} times", state.button_clicks);
            }))
            .with_text("Increment"),

        Button::new("Test")
            .with_on_click(Box::new(|state: &mut State| {
                state.button_clicks = 0;
                println!("Resetting counter");
            }))
            .with_text("Reset"),

        Text::new("Test", "Text Widget")
            .with_color(255, 255, 255),

        Button::new("Test")
            .with_on_click(Box::new(|state: &mut State| {
                example_callback(state);
            }))
            .with_text("Button")
    )
    .with_fixed_size(300, 400)
    .centered();

    // TODO: This must allow some mechanism for dynamic views
    //       Consider requiring a function which takes the state and returns a view
    main_window.start(test_vstack);
}

fn example_callback(_state: &mut State) {
    println!("This is a function");
}