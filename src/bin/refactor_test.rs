#[macro_use]
extern crate RustUI;
macro_imports!();

use RustUI::Window;

use RustUI::widgets::{Button, Text};
use RustUI::views::{HStack, VStack};
use RustUI::colors;

#[derive(Clone, PartialEq)]
struct State {
    counter: i16,
}

impl RustUI::backend::system::state::GenerateView<State> for State {
    fn generate_view(&self) -> Box<dyn View<State>> {
        // Dynamic view example. This would realistically be done by calling generator functions
        let view = if self.counter % 2 == 0 {
            VStack!(
                Text::new("test", "Even")
                    .with_color(colors::WHITE),

                HStack!(
                    Button::new("test2")
                        .with_text("Keep Even")
                        .with_on_click(|state: &mut State| {
                            state.counter += 2;
                        }),

                    Button::new("test3")
                        .with_text("Make Odd")
                        .with_on_click(|state: &mut State| {
                            state.counter += 1;
                        }),

                    Text::new("counter", format!("Number: {}", self.counter).as_str())
                    .with_color(colors::WHITE)
                )
            )
        } else {
            VStack!(
                Text::new("counter", format!("Number: {}", self.counter).as_str())
                    .with_color(colors::WHITE),

                HStack!(
                    Button::new("test4")
                        .with_text("Keep Odd")
                        .with_on_click(|state: &mut State| {
                            state.counter += 2;
                        }),

                    Button::new("test5")
                        .with_text("Make Even")
                        .with_on_click(|state: &mut State| {
                            state.counter += 1;
                        })
                ),

                Text::new("test6", "Odd")
                    .with_color(colors::WHITE)
            )
            .fixed_height(200)
            .fixed_width(300)
        };

        Box::new(view)
    }
}

fn main() {
    let mut state = State {counter: 0};
    let main_window = Window::init("Test", &mut state);

    main_window.start();
}