/*

This file will serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;

// TODO: A number of modules/components need to be re-exported. Maybe in lib.rs for convenience
//  An import macro might also be useful for convenience
use RustUI::backend::system::window::Window;
use RustUI::backend::system::state::GenerateView;
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
            counter: 1,
            is_locked: false,
        }
    }
}

impl<T> GenerateView<T, State> for State {
    fn generate_view(&self) -> Box<dyn View<State>> {

        // TODO: Need a way to handle loops/if statements for view generation (within macros)
        let view = VStack!(
            // New method
            Text::new("CounterText", format!("Counter: {}", self.counter).as_str())
                .with_color(colors::WHITE),

            // Old method
            // Text::new("CounterText", "Counter: 0")
            //     .with_text_update(|state: &State| {
            //         format!("Counter: {}", state.counter)
            //     })
            //     .with_color(colors::WHITE),

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

        Box::new(view)
    }
}

fn main() {
    let mut app_state = State::new();

    let mut main_window = Window::init("RustUI Testing", &mut app_state);
    main_window.set_icon("./res/logo/temp_logo_low_quality.bmp");

    main_window.start();
}

fn example_callback(_state: &mut State) {
    println!("This is a function");
}