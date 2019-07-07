use super::widgets::*;

// extern crate proc_macro;
// use proc_macro::TokenStream;

pub type View = Vec<Box<Widget>>;


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