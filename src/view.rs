/* TODO: Nested View + Widget trees

Consider some implementation such as the example enum below.

The following resources may enable type checking while nesting:
https://stackoverflow.com/questions/34214136/how-do-i-match-the-type-of-an-expression-in-a-rust-macro
https://doc.rust-lang.org/std/any/index.html

Furthermore, consider creating a shared trait for both views and widgets.
This trait could include a method for obtaining the object type, eliminating the need for
the awkward enum below.

*/


use sdl2::ttf;
use sdl2::rect::Rect;
use std::collections::HashMap;
use super::widgets::*;
use super::font::{FontParams, Fonts};


// TODO: This
pub trait ViewComponent<T> {
    fn get_text(&self) -> Option<Text<T>> {
        None
    }
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_center(&self) -> (u32, u32);
}

pub type SubView<T> = Vec<Box<Widget<T>>>;

// impl Iterator for SubView<T> {

// }

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
    // Map of user-assigned widget names -> widget
    // component_map: HashMap<&'static str, SubView<T>>,

    pub subview: SubView<T>,
    pub view_width: u32,
    pub view_height: u32,
    pub fixed_size: bool,
    pub default_padding: u32,
    pub alignment: Alignment,
}

impl<T> View<T> {
    // TODO: Build the view here rather than within the macro.
    pub fn init(&mut self, ttf_context: &ttf::Sdl2TtfContext) {

        // TODO: How to extend this lifetime and implement for text rendering?
        let mut font_manager = Fonts::new();

        // Step 1 -> Size text surfaces
        for view in &mut self.subview {
            // If the view has a text component, obtain its surface size
            if let Some(text_component) = view.text_component() {
                font_manager.load_font(ttf_context, &text_component.font);
                let text_surface_size = font_manager.size_surface(&text_component.font, &text_component.text);
                view.assign_text_dimensions(text_surface_size);
            }
        }

        // Step 2 -> Align contents
        match self.alignment {
            Alignment::Center => { // Translate each widget to be centered
                for widget in &mut self.subview {
                    let new_x = (self.view_width / 2) as i32 - (widget.draw_width() / 2) as i32;
                    // println!("Translating from {} to {}", widget.rect().x(), new_x as i32 - widget.rect().x());
                    widget.translate(new_x - widget.rect().x(), 0);
                }
            }
            // TODO: implement the rest
            _ => {}
        }

    }

    /// Lock the window's size (stops dynamic size adjustments)
    pub fn fixed_size(mut self, width: u32, height: u32) -> Self {
        self.view_width = width;
        self.view_height = height;
        self.fixed_size = true;
        self
    }

    /// Lock the window's width
    pub fn fixed_width(mut self, width: u32) -> Self {
        self.view_width = width;
        self
    }

    /// Align a view's widgets
    // TODO: Need some 'get_center' function for widgets with text (e.g. checkbox)
    pub fn align_content(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
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
                alignment: Alignment::Left,
            }
        }
    };
}

#[macro_export]
macro_rules! HStack {
    ( $($x:expr), + ) => {
        {
            let mut view = SubView::new();

            let default_padding = 10;
            // Current draw location
            let mut current_x = 0;

            let mut max_y: u32 = 0;

            // FIXME: This should be handled by Button::new(str) and derive from the string
            let mut current_id = 0;

            // TODO: How to account for user-defined sizes, positions, etc?
            $(
                let widget = $x
                    .with_id(current_id)
                    .place( current_x + default_padding, default_padding);

                current_x += widget.rect().width() as i32 + default_padding;

                // Note that widget gets moved here (can no longer be accessed within this scope)
                view.push(Box::new(widget));

                current_id += 1;
            )+

            for widget in &view {
                let required_y = widget.rect().y() as u32 + widget.rect().height() as u32;
                if required_y > max_y {
                    max_y = required_y;
                }
            }

            View {
                subview: view,
                view_width: current_x as u32 + default_padding as u32,
                view_height: max_y + default_padding as u32,
                fixed_size: false,
                default_padding: default_padding as u32,
                alignment: Alignment::Left,
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
                alignment: Alignment::Left,
            }
        }
    };
}
