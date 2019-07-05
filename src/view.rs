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
                // This will happen to each element
                view.push(Box::new($x));
            )+
            
            view
        }
    };
}