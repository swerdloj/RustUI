use super::widgets::*;

// extern crate proc_macro;
// use proc_macro::TokenStream;

pub type SubView<T> = Vec<Box<Widget<T>>>;

// TODO: subview should be able to iterate through all widgets and nested view widgets
//       This capability must be reflected in the backend as well
pub struct View<T> {
    pub subview: SubView<T>,
    pub view_width: u32,
    pub view_height: u32,
    pub fixed_size: bool,
}

impl<T> View<T> {
    /// Lock the window's size (stops dynamic size adjustments)
    pub fn with_fixed_size(mut self, width: u32, height: u32) -> Self {
        self.fixed_size = true;
        self
    }
}


// Macro assistance: https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html

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
            let mut view = SubView::new();

            // Begin repetition
            $(
                // TODO: Widget positioning, padding, etc. will be determined here
                
                // This will happen to each element
                view.push(Box::new($x));
            )+
            
            // and this is the output
            View {
                subview: view,
                view_width: 800,
                view_height: 600,
                fixed_size: true,
            }
        }
    };
}

#[macro_export]
/// Vertical layout (space widgets vertically)
macro_rules! VStack {
    ( $($x:expr), + ) => {
        {
            let mut view = SubView::new();

            let default_padding = 10;
            // Current draw location
            let mut current_y = 0;

            let mut max_x: u32 = 0;

            // FIXME: This should be handled by Button::new(str) and derive from the string
            let mut current_id = 0;

            // TODO: How to account for user-defined sizes, positions, etc?
            $(
                view.push(Box::new(
                    $x
                    // TODO: Consider replacing .with_rect with ".at". This will not override rect dimensions.
                    .with_id(current_id)    
                    .place(default_padding, current_y + default_padding)
                ));
                // TODO: Fix the below line
                // current_y += $x.rect().height() as i32 + default_padding;
                current_y += 40 as i32 + default_padding;
                current_id += 1;
            )+

            for widget in &view {
                let required_x = widget.rect().x() as u32 + widget.rect().width() as u32;
                if required_x > max_x {
                    max_x = required_x;
                }
            }

            View {
                subview: view,
                view_width: max_x + default_padding as u32,
                view_height: current_y as u32 + default_padding as u32,
                fixed_size: false,
            }
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