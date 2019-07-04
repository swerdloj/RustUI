/*

This file were serve as a usability test.
It will be structured as though creating a real project using this library.

*/

extern crate RustUI;
extern crate sdl2;

use RustUI::backend::system::window;
use RustUI::widgets::*;
use sdl2::rect::Rect;

fn main() {
    let main_window = window::Window::init("Test");

    let example_button = Button {
        rect: Rect::new(100, 200, 100, 40),
        // on_click: &|| {println!("Test")}, // Note there are 2 ways to pass callbacks
        on_click: &example_callback,
    };

    main_window.start(example_button);
}

fn example_callback() {
    println!("example_callback was triggered");
}