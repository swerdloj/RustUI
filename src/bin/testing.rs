/*

This file will serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;
macro_imports!();

use RustUI::backend::system::state::GenerateView;

use RustUI::{Window, colors, Alignment};
use RustUI::widgets::{Text, Button, TextBox, CheckBox};
use RustUI::views::{HStack, VStack};


#[derive(Clone, PartialEq)]
struct State {
    counter: i16,
    is_locked: bool,
    text_input: String,
}

impl State {
    fn new() -> Self {
        State {
            counter: 0,
            is_locked: false,
            text_input: String::new(),
        }
    }
}

impl GenerateView<State> for State {
    fn generate_view(&self) -> Box<dyn View<State>> {
        // TODO: Need a way to handle loops/if statements for view generation (within macros)
        let view = VStack!(
            Text::new("CounterText", 
                    format!("Counter: {}", self.counter).as_str())
                .with_color(colors::WHITE),

            HStack!(
                Button::new("IncrementButton")
                    .with_text("++")
                    .with_on_click(|state: &mut State| {
                        if !state.is_locked {
                            state.counter += 1;
                        }
                    }),

                Button::new("DecrementButton")
                    .with_text("--")
                    .with_on_click(|state: &mut State| {
                        if !state.is_locked {
                            state.counter -= 1;
                        }
                    })
            )
            .padding(10, 10, 5, 0),

            // TODO: How can input text persist between view cycles without user-defined variable?
            // TODO: Avoid using clone
            // TODO: Account for text 'submission' such as enter key press
            TextBox::new("Test", self.text_input.clone())
                .with_default_text("Number...")
                .with_on_text_changed(|state: &mut State, text| {
                    state.text_input = text;
                })
                .with_on_text_submit(|state: &mut State, text| {
                    if !state.is_locked {
                        set_counter_from_string(state, text);
                        state.text_input.clear();
                    }
                }),

            CheckBox::new("LockCounter", self.is_locked)
                .with_text("Lock")
                .with_on_check(|state: &mut State, is_checked| {
                    state.is_locked = is_checked;
                }),

            Button::new("ResetCounter")
                .with_on_click(|state: &mut State| {
                    if !state.is_locked {
                        state.counter = 0;
                        println!("Resetting counter");
                    } else {
                        println!("The counter is locked");
                    }
                })
                .with_text("Reset")
        )
        // .fixed_width(400)
        .alignment(Alignment::Center);

        Box::new(view)
    }
}

fn main() {
    let mut app_state = State::new();

    let mut main_window = Window::init("RustUI Testing", &mut app_state);
    main_window.set_icon("./res/logo/temp_logo_low_quality.bmp");

    main_window.start();
}

/// Example helper function
/// - Parses `String` as integer
/// - Sets state.counter if `String` is valid number
fn set_counter_from_string(state: &mut State, text: String) {
    // Note how Rust enforces input safety
    if let Ok(number) = text.parse() {
        state.counter = number;
        println!("Setting counter to {}", number);
    } else {
        println!("Warning: '{}' is either not a number or exceeds i16 capacity", text);
    }
}