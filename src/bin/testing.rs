/*

This file will serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;
macro_imports!();

use RustUI::state::GenerateView;
use RustUI::{Window, colors, Alignment, Orientation};
use RustUI::widgets::{Text, Button, TextBox, CheckBox, ScrollBar, Image};
use RustUI::views::{HStack, VStack, Overlay};
use RustUI::components::{Divider};


#[derive(Clone, PartialEq)]
struct State {
    counter: i32,
    is_locked: bool,
    text_input: String,
    slider_val: i32,

    show_overlay: bool,
}

impl State {
    fn new() -> Self {
        State {
            counter: 0,
            is_locked: false,
            text_input: String::new(),
            slider_val: 1,

            show_overlay: false,
        }
    }
}

impl GenerateView<State> for State {
    fn generate_view(&self) -> Box<dyn View<State>> {
        // TODO: Need a way to handle loops/if statements for view generation (within macros)
        let mut view = VStack!(
            // TODO: Test different images & formats
            Image::new("TestImage", "./res/logo/temp_logo_low_quality.bmp", (100, 100))
                .with_hover_shade()
                .with_on_click(|_state: &mut State| {
                    println!("Clicked the image");
                }),

            Text::new("CounterText", 
                    format!("Counter: {}", self.counter).as_str())
                .with_point_size(35)
                .with_color(colors::WHITE),

            Divider::new(Orientation::Horizontal),

            ScrollBar::new("TestScroll", 0, 10, self.slider_val)
                .with_length(200)
                .with_on_value_changed(|state: &mut State, value| {
                    state.slider_val = value;
                }),

            HStack!(
                Button::new("AddButton")
                    .with_text("+")
                    .with_on_click(|state: &mut State| {
                        if !state.is_locked {
                            state.counter += state.slider_val;
                        }
                    }),

                Text::new("ScrollText", &format!("{}", self.slider_val))
                    .with_color(colors::WHITE)
                    .with_point_size(50)
                    .center(),
                
                Button::new("SubtractButton")
                    .with_text("-")
                    .with_on_click(|state: &mut State| {
                        if !state.is_locked {
                            state.counter -= state.slider_val;
                        }
                    })
            )
            .padding(10, 10, 5, 0),

            // TODO: How can input text persist between view cycles without user-defined variable?
            TextBox::new("Test", &self.text_input)
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
                .with_text("Reset"),

            Button::new("OverlayButton")
                .with_text("Overlay")
                .with_on_click(|state: &mut State| {
                    state.show_overlay = !state.show_overlay;
                })
        )
        // .fixed_width(400)
        .alignment(Alignment::Center);

        // TODO: Handle `if` statements properly.
        //  Macros should be able to handle different return types so long as
        //  IntoViewComponent is satisfied (or if return type is `()`)
        if self.show_overlay {
            view.overlay(
                VOverlay!(
                    VStack!(
                        Text::new("OverlayText", "This is an overlay test")
                            .with_color(colors::WHITE),
                        TextBox::new("OverlayTextBox", "")
                            .with_default_text("Overlay...")
                    )
                )
            )
        }

        return Box::new(view);
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
        println!("Warning: '{}' is either not a number or exceeds i32 capacity", text);
    }
}