use super::widgets::*;

// extern crate proc_macro;
// use proc_macro::TokenStream;

pub type View<T> = Vec<Box<Widget<T>>>;


// Macro assistance: https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html

// TODO: Make Widget Rect an Option, then assign the rect here according to view properties (padding, etc.)
#[macro_export]
macro_rules! example_view {
    (
        // Repetition
        $(
            // Where each element is an item
            $x:expr
        )
        // seperated by commas
        , 
        // one or more times
        +
    ) => {
        {
            let mut view = View::new();

            // Begin repetition
            $(
                // TODO: Widget positioning, padding, etc. will be determined here
                
                // This will happen to each element
                view.push(Box::new($x));
            )+
            
            // and this is the output
            view
        }
    };
}

#[macro_export]
/// Vertical layout (space widgets vertically)
macro_rules! VStack {
    ( $($x:expr), + ) => {
        {
            let mut view = View::new();

            let default_button_width = 100;
            let default_button_height = 40;

            let default_padding = 10;
            // Current draw location
            let mut current_y = 0;

            // FIXME: This should be handled by Button::new(str) and derive from the string
            let mut current_id = 0;

            // TODO: How to account for user-defined sizes, positions, etc?
            $(
                view.push(Box::new(
                    $x
                    .with_rect(Rect::new(default_padding, current_y + default_padding, 
                                         default_button_width, default_button_height))
                    .with_id(current_id)    
                ));
                current_y += default_button_height as i32 + default_padding;
                current_id += 1;
            )+

            view
        }
    };
}

    // Note that traits can be both overwritten and extended (by calling super?)

pub mod views {
    // TODO: Worry about this after widgets are implemented (Also move this to another file)
    trait View {
        // TODO: How to work this idea into the program?
        fn generate() {
            // Build the view (would the user implement this function? e.g.: don't provide a default?)
        }
    }
}