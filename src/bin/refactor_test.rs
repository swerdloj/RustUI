#[macro_use]
extern crate RustUI;

use RustUI::backend::system::window::Window;
// TODO: Work on imports & namespaces
use RustUI::view_components::{
    views::{VStack, HStack, view::{View, Alignment}},
    widgets::Button,
    WidgetOrView,
    ViewComponent,
};

fn main() {
    let mut state = 7;

    let main_window = Window::init("Test", &mut state);

    let view = VStack!(
        Button::new("test1")
            .with_text("VS1 1/3"),
        Button::new("test2")
            .with_text("VS1 2/3"),
        Button::new("test3")
            .with_text("VS1 3/3"),

        // Text::new("Test", "Text")

        HStack!(
            Button::new("test4")
                .with_text("HS1 1/3"),
            Button::new("test5")
                .with_text("HS1 2/3"),
            Button::new("test6")
                .with_text("HS1 3/3"),

            // Should be 4th element in HStack
            VStack!(
                Button::new("test7")
                    .with_text("VS2 1/3"),
                Button::new("test8")
                    .with_text("VS2 2/3"),
                // FIXME: Something is not accounting for nested view width
                // HStack!(
                //     Button::new("empty")
                //         .with_text("Nested1"),
                //     Button::new("empty")
                //         .with_text("Nested2")
                // ),
                Button::new("test9")
                    .with_text("VS2 3/3")
            )
        )
        // ,
        // Button::new("asdf")
        //     .with_text("VS1 End")
    )
    .alignment(Alignment::Center);
    // .fixed_width(500)
    // .fixed_height(400);

    // TODO: Refactor `backend.rs` to implement the new system
    main_window.start(view);
}