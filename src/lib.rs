/*
Rust GUI library
Proposed syntax:

--Begin example--

extern crate rust_gui;

use rust_gui::views::{ View, VStack, HStack, Text }; // Pre-made views to choose from (like SwiftUI)
use rust_gui::events::Event; // Event feed like in SDL
use rust_gui::widgets::*; // Buttons, checkboxes, etc. (like tkinter)

fn main() {
    let window = rust_giu::init(rust_gui::Accelerated); // Use enums to simplify options (maybe use rust_gui::Enums ?)
    
    ... more setup...

    // View creation (Views can be nested)
    let example_view: View = VStack!( // Use procedural macros to make life easier?
        HStack!(
            Text("This is an example")
            .size(14)
            .font("Arial")
            .bold(),

            Text("This is more text")
            .alignment(...::Right),

            ...
        ).padding(10), // Which units to use, percentage?

        // Add the button using a struct
        Button { on_click: some_function, 
                 on_hover: animation_function,
                 width: 14
                 .. }  // Take advantage of Rust's pattern matching
        .padding_top(40),
        ...
    );
}
--End example--

Utilizing Rust for Dynamic Views:

let some_bool = false;
let some_other_bool = true;

let y: i32 = {
        let mut return_value: i32 = 12;
        if some_bool {
            return_value = return_value - 12;
        } else if some_other_bool {
            return_value = return_value + 5;
        }
        return_value
};

Expressions are allowed inside declarations using "{}". When generating a view, this can be utilized to create dynamic views.
Or, a function can be used to generate the view which allows for passing of parameters.
See /bin/nested_structure_example.rs for dynamic, nested declarations

*/

// TODO: Create a build file & include SDL libraries

// TODO: IMPORTANT - Document the code according to the following:
// https://doc.rust-lang.org/rust-by-example/meta/doc.html

// For design help, see:
// https://github.com/gyscos/cursive

pub mod backend;
pub mod widgets;
pub mod view;

pub mod rust_gui {

}