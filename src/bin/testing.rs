/*

This file were serve as a usability test.
It will be structured as though creating a real project using this library.

*/

#[macro_use]
extern crate RustUI;
extern crate sdl2;

use RustUI::backend::system::window;
use RustUI::widgets::*;
use RustUI::view::{View};
use sdl2::rect::Rect;
// use sdl2::pixels::Color;

fn main() {
    let main_window = window::Window::init("Test");

    // let mut button_clicks: u16 = 0;

    // TODO: Need some way to pass mutable state around
    // let example_state_modifier = || {button_clicks = button_clicks + 1;};

    // TODO: the view macro must handle default layout/padding according to the view type
    let test_view = example_view!(
        Button::new("Test")
            .with_id(1)
            .with_on_click(&button1_callback)
            .with_rect(Rect::new(100, 200, 100, 40)), 

        Button::new("Test")
            .with_id(2)
            .with_on_click(&button2_callback)
            .with_rect(Rect::new(210, 200, 100, 40)),

        Button::new("Test")
            .with_id(3)
            .with_on_click(&button3_callback)
            .with_rect(Rect::new(100, 250, 100, 40)),

        Button::new("Test")
            .with_id(4)
            .with_on_click(&|| {println!("Button 4 was clicked");})
            .with_rect(Rect::new(210, 250, 100, 40))
        );

    // TODO: This should also accept the application state as mutable
    //       Must allow buttons, etc. to modify state
    main_window.start(test_view);
}

fn button1_callback() {
    println!("Button 1 was clicked");
}

fn button2_callback() {
    println!("Button 2 was clicked");
}

fn button3_callback() {
    println!("Button 3 was clicked");
}