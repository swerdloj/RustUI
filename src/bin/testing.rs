/*

This file were serve as a usability test.
It will be structured as though creating a real project using this library.

*/

extern crate Practicum;

use Practicum::backend::system::window;

fn main() {
    let main_window = window::Window::init("Test");

    main_window.start();
}