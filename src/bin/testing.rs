/*

This file will serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;

use RustUI::backend::system::window::Window;
use RustUI::view_components::{
    // TODO: User should not need to import these
    WidgetOrView, ViewComponent,
    views::view::{View, Alignment},

    views::{VStack, HStack},
    widgets::{Text, Button, CheckBox, widget::colors}
};

struct State {
    counter: i16,
    is_locked: bool,
}

impl State {
    fn new() -> Self {
        State {
            counter: 0,
            is_locked: false,
        }
    }
}

// TODO: Implement trait requirement for backend
impl<T> RustUI::backend::system::state::GenerateView<T, State> for State {
    fn generate_view(&self) -> Box<dyn View<State>> {
        let view = VStack!(
            Button::new("test")
                .with_on_click(|state: &mut State| {
                    state.counter += 1;
                }),
            
            Text::new("test2", "testing")
                .with_text_update(|state: &State| {
                    format!("{}", state.counter)
                })
        );

        Box::new(view)
    }
}

fn main() {
    let mut app_state = State::new();

    let mut main_window = Window::init("RustUI Testing", &mut app_state);
    main_window.set_icon("./res/logo/temp_logo_low_quality.bmp");

    let test_view = VStack!(
        Text::new("CounterText", "Counter: 0")
            // FIXME: When updating text, the text component must be resized
            .with_text_update(|state: &State| {
                format!("Counter: {}", state.counter)
            })
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