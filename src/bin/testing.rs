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
use sdl2::pixels::Color;

fn main() {
    let main_window = window::Window::init("Test");

    let example_button1 = Button {
        id: 10,
        rect: Rect::new(100, 200, 100, 40),
        primary_color: Color::RGB(240, 240, 200),
        secondary_color: Color::RGB(100, 100, 100),
        hover_color: Color::RGB(200, 200, 200),
        // on_click: &|| {println!("Test")}, // Note there are 2 ways to pass callbacks
        on_click: Some(&button1_callback),
    };

    let example_button2 = Button {
        id: 12,
        rect: Rect::new(210, 200, 100, 40),
        primary_color: Color::RGB(240, 240, 200),
        secondary_color: Color::RGB(100, 100, 100),
        hover_color: Color::RGB(200, 200, 200),
        on_click: Some(&button2_callback),
    };

    let example_button3 = Button::new("Test")
                          .with_on_click(&button3_callback)
                          .with_rect(Rect::new(100, 250, 100, 40));

    let test_view = example_view!(example_button1, 
                                  example_button2,
                                  example_button3
                                 );


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