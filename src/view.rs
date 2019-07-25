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
    pub default_padding: u32,
}

impl<T> View<T> {
    /// Lock the window's size (stops dynamic size adjustments)
    pub fn with_fixed_size(mut self, width: u32, height: u32) -> Self {
        self.view_width = width;
        self.view_height = height;
        self.fixed_size = true;
        self
    }

    /// Center a view within a fixed-size window
    pub fn centered(mut self) -> Self {
        if !self.fixed_size {
            println!("Warning: Centering does not affect dynamically sized windows");
            return self;
        }

        let center = self.view_width / 2;

        for widget in &mut self.subview {
            // Left-most draw position
            let widget_x = widget.rect().x() as u32 - self.default_padding;
            let new_x = center - (widget.rect().width() / 2) as u32 + widget_x;

            println!("Moving widget at x={} to x={}", widget.rect().x(), new_x);

            widget.translate(new_x as i32 - widget.rect().x(), 0);
        }

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
                default_padding: 10,
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
                let widget = $x
                    .with_id(current_id)
                    .place(default_padding, current_y + default_padding);

                current_y += widget.rect().height() as i32 + default_padding;

                // Note that widget gets moved here (can no longer be accessed within this scope)
                view.push(Box::new(widget));

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
                default_padding: default_padding as u32,
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