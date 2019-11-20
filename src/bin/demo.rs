#[macro_use]
extern crate RustUI;

// Imports
use RustUI::{
    View, Window, Alignment, Orientation, colors,
    state::GenerateView,
    widgets::*,
    views::*,
    components::*,
};

// App state
#[derive(Clone, PartialEq)]
struct State {

}

// App state methods
impl State {
    pub fn new() -> Self {
        State {

        }
    }
}

// State-dependent view generation
impl GenerateView<State> for State {
    fn generate_view(&self) -> Box<dyn View<State>> {
        let view = VStack!(
            Text::new("TextWidget", "Demo")
                .with_point_size(35)
                .with_color(colors::WHITE),

            Button::new("ButtonWidget")
                .with_text("Demo")
        );

        Box::new(view)
    }
}

// Initialize/run
fn main() {
    let mut state = State::new();

    let mut window = Window::init("RustUI Demo", &mut state);
    window.set_icon("./res/logo/temp_logo_low_quality.bmp");
    
    window.start();
}




/*

Image::new("TestImage", "./res/logo/temp_logo_low_quality.bmp", (100, 100))

*/