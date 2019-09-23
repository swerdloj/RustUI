/*

This file will serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;

use RustUI::backend::system::window::Window;
use RustUI::view_components::{
    WidgetOrView, ViewComponent,
    views::view::{View, Alignment},
    views::{VStack, HStack},
    widgets::{Text, Button, CheckBox, widget::colors}
};

struct State {
    button_clicks: i16,
    is_locked: bool,
}

impl State {
    fn new() -> Self {
        State {
            button_clicks: 0,
            is_locked: false,
        }
    }
}

fn main() {
    let mut app_state = State::new();

    let mut main_window = Window::init("RustUI Testing", &mut app_state);
    main_window.set_icon("./res/logo/temp_logo_low_quality.bmp");

    let test_view = VStack!(
        Text::new("CounterText", "Counter: 0")
            // TODO: When updating text, the text component must be resized
            .with_text_update(|state: &State| {
                format!("Counter: {}", state.button_clicks)
            })
            .with_color(colors::WHITE),

        HStack!(
            Button::new("IncrementButton")
                .with_text("++")
                .with_on_click(|state: &mut State| {
                    if !state.is_locked {
                        state.button_clicks += 1;
                    }
                }),

            Button::new("DecrementButton")
                .with_text("--")
                .with_on_click(|state: &mut State| {
                    if !state.is_locked {
                        state.button_clicks -= 1;
                    }
                })
        ),

        Button::new("ResetCounter")
            .with_on_click(|state: &mut State| {
                if !state.is_locked {
                    state.button_clicks = 0;
                    println!("Resetting counter");
                } else {
                    println!("The counter is locked");
                }
            })
            .with_text("Reset"),

        CheckBox::new("LockCounter")
            .with_text("Lock")
            .with_on_check(|state: &mut State, is_checked| {
                state.is_locked = is_checked;
            }),

        Text::new("Test", "Text Widget")
            .with_color(colors::WHITE),

        Button::new("ExampleButton")
            .with_on_click(example_callback) // Can simply pass in regular functions
            .with_text("Button")        
    )
    // .fixed_width(400)
    .alignment(Alignment::Center);

    // TODO: This must allow some mechanism for dynamic views
    //       Consider requiring a function which takes the state and returns a view
    main_window.start(test_view);
}

fn example_callback(_state: &mut State) {
    println!("This is a function");
}