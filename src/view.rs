use super::widgets::*;

// extern crate proc_macro;
// use proc_macro::TokenStream;

pub type SubView<T> = Vec<Box<Widget<T>>>;

/// View alignments
/// ## Alignments
/// * `Left` - Align each widget to the left within its view (default)
/// * `Center` - Center each widget within its view
pub enum Alignment {
    Center,
    Left,
    // TODO: Will this be used?
    // Right,
}

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

    pub fn with_fixed_width(mut self, width: u32) -> Self {
        self.view_width = width;
        self
    }

    /// Align a view's widgets
    // TODO: Need some 'get_center' function for widgets with text (e.g. checkbox)
    pub fn align_content(mut self, alignment: Alignment) -> Self {
        match alignment {
            Alignment::Center => {
                for widget in &mut self.subview {
                    let new_x = (self.view_width / 2) - (widget.rect().width() / 2);
                    widget.translate(new_x as i32 - widget.rect().x(), 0);
                }
            }
            // TODO: implement the rest
            _ => {}
        }

        self
    }
}


// Macro assistance: https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html


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
