#[macro_use]
extern crate RustUI;

use RustUI::backend::system::window::Window;
// TODO: Work on imports & namespaces
use RustUI::view_components::{
    views::{
        VStack, HStack, view::{View, Alignment}
    },
    widgets::{
        Button, CheckBox, Text, widget::colors,
    },
    WidgetOrView,
    ViewComponent,
};

fn main() {
    let mut state = 7;

    let main_window = Window::init("Test", &mut state);

    let view = VStack!(
        Text::new("test0", "Start")
            .with_color(colors::WHITE),
        Button::new("test1")
            .with_text("VS1 1/3"),
        Button::new("test2")
            .with_text("VS1 2/3"),
        Button::new("test3")
            .with_text("VS1 3/3"),

        HStack!(
            VStack!(
                Button::new("test4")
                    .with_text("VS3 1/2"),
                Button::new("test5")
                    .with_text("VS3 2/2")
            ),
            Button::new("test6")
                .with_text("HS1 1/2"),
            Button::new("test7")
                .with_text("HS1 2/2"),

            // Should be last element in HStack
            VStack!(
                Button::new("test8")
                    .with_text("VS2 1/3"),
                Button::new("test9")
                    .with_text("VS2 2/3"),
                // FIXME: Subvew width still needs to be accounted for
                // HStack!(
                //     Button::new("empty")
                //         .with_text("Nested1"),
                //     Button::new("empty")
                //         .with_text("Nested2")
                // ),
                Button::new("test10")
                    .with_text("VS2 3/3")
            )
        ),

        CheckBox::new("test11")
            .with_text("End")
    )
    .fixed_height(420)
    .alignment(Alignment::Center);

    main_window.start(view);
}