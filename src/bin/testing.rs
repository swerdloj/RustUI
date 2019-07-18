/*

This file were serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;
extern crate sdl2;

use RustUI::backend::system::window::Window;
use RustUI::backend::system::state::{ApplicationState};
use RustUI::widgets::*;
use RustUI::view::{View};
use sdl2::rect::Rect;
// use sdl2::pixels::Color;

struct State {
    button_clicks: u16,
}


fn main() {
    let mut app_state = State { button_clicks: 0 };

    let main_window: Window<State> = Window::init("Test", &mut app_state);

    // TODO: the view macro must handle default layout/padding according to the view type
    let test_vstack = VStack!(
        Button::new("Test")
            .with_on_click(Box::new(|state: &mut State| {
                state.button_clicks += 1;
                println!("Clicked the button {} times", state.button_clicks);
            })),

        Button::new("Test")
            .with_on_click(Box::new(|_| {
                println!("This doesn't increment anything");
            })),

        Button::new("Test")
            .with_on_click(Box::new(|state: &mut State| {
                example_callback(state);
            }))
    );

    // TODO: This must allow some mechanism for dynamic views
    main_window.start(test_vstack);
}

fn example_callback(state: &mut State) {
    println!("This is a function");
}