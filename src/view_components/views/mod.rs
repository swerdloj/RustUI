// TODO: Where should items such as dividers and spacers go? Would these be views?

pub mod view;
pub mod vstack;
pub mod hstack;

pub type VStack<T> = vstack::VStack<T>;
pub type HStack<T> = hstack::HStack<T>;


// ========================== Example macro ========================== //

/// Example view macro for reference
#[allow(unused_macros)]
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
    ) => { // Contain the macro
        { // What the macro expands to
            // This happens only once
            let mut view = Vec::new();

            // Begin repetition
            $(               
                // This will happen to each element
                view.push(Box::new($x));
            )+
            
            // and this is the output
            View::new(view)
        }
    };
}