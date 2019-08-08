use sdl2::ttf;
use sdl2::rect::Rect;
use std::collections::HashMap;
use super::widgets::*;
use super::font::{FontParams, Fonts};

// ========================== WidgetOrView enum ========================== //

/// Contains either a Widget or a View. Handle via `match`.
pub enum WidgetOrView<T> {
    Widget(Box<Widget<T>>),
    View(View<T>),
}

// ========================== ViewComponent trait ========================== //

// TODO: This
pub trait ViewComponent<T> {
    // fn get_width(&self) -> u32;
    // fn get_height(&self) -> u32;
    // fn get_center(&self) -> (u32, u32);
    fn as_component(self) -> WidgetOrView<T>;
}

// ========================== Alignment enum ========================== //

/// View alignments
/// ## Alignments
/// * `Left` - Align each widget to the left within its view (default)
/// * `Center` - Center each widget within its view
/// * `Right` - TODO: This
pub enum Alignment {
    Center,
    Left,
    Right,
}

// ========================== View struct ========================== //

// TODO: subview should be able to iterate through all widgets and nested view widgets
//       This capability must be reflected in the backend as well
pub struct View<T> {
    // Map of user-assigned widget names -> widget
    pub component_map: HashMap<u32, Box<Widget<T>>>,

    pub components: Vec<WidgetOrView<T>>,

    pub view_width: u32,
    pub view_height: u32,
    pub fixed_size: bool,
    pub default_padding: u32,
    pub alignment: Alignment,
}

impl<T> ViewComponent<T> for View<T> {
    fn as_component(self) -> WidgetOrView<T> {
        WidgetOrView::View(self)
    }
}

impl<T> View<T> {
    /// Returns a `vec` of mutable references to all widgets within a view
    // TODO: This should use the hashmap
    pub fn widgets_mut(&mut self) -> Vec<&mut Box<Widget<T>>> {
        let mut widgets = Vec::new();

        for item in &mut self.components {
            match item {
                WidgetOrView::Widget(widget) => {
                    widgets.push(widget);
                }
                WidgetOrView::View(nested_view) => {
                    // Recursively iterate through all nested views
                    widgets.append(&mut nested_view.widgets_mut());
                }
            }
        }

        widgets
    }

    /// Returns a `vec` of references to all widgets within a view
    // TODO: This should use the hashmap
    pub fn widgets(&self) -> Vec<&Box<Widget<T>>> {
        let mut widgets = Vec::new();

        for item in &self.components {
            match item {
                WidgetOrView::Widget(widget) => {
                    widgets.push(widget);
                }

                WidgetOrView::View(nested_view) => {
                    // Recursively iterate through all nested views
                    widgets.append(&mut nested_view.widgets());
                }
            }
        }

        widgets
    }

    // TODO: Build the view here rather than within the macro.
    pub fn init(&mut self, ttf_context: &ttf::Sdl2TtfContext) {

        // TODO: How to extend this lifetime and implement for text rendering?
        let mut font_manager = Fonts::new();

        for item in &mut self.components {
            match item {
                WidgetOrView::Widget(widget) => {
                    // If the widget has a text component, obtain its surface size
                    if let Some(text_component) = widget.text_component() {
                        font_manager.load_font(ttf_context, &text_component.font);
                        let text_surface_size = font_manager.size_surface(&text_component.font, &text_component.text);
                        widget.assign_text_dimensions(text_surface_size);
                    }
                }

                WidgetOrView::View(nested_view) => {
                    nested_view.init(ttf_context);
                }
            }
        }

        // Step 2 -> Align contents
        match self.alignment {
            Alignment::Center => { // Translate each widget to be centered
                for item in &mut self.components {
                    match item {
                        WidgetOrView::Widget(widget) => {
                            let new_x = (self.view_width / 2) as i32 - (widget.draw_width() / 2) as i32;
                            // println!("Translating from {} to {}", widget.rect().x(), new_x as i32 - widget.rect().x());
                            widget.translate(new_x - widget.rect().x(), 0);
                        }

                        WidgetOrView::View(subview) => {

                        }
                    }
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

// TODO: Move view building out of the macro and into View's init() function
//       This will help considerably with future refactoring & view nesting



// ========================== VStack macro ========================== //

#[macro_export]
/// Vertical layout (space widgets vertically)
macro_rules! VStack {
    ( $($x:expr), + ) => {
        {
            let mut view = Vec::new();

            let default_padding = 10;
            // Current draw location
            let mut current_y = 0;

            let mut max_x: u32 = 0;

            // FIXME: This should be handled by Widget::new(str) and derive from the string
            let mut current_id = 0;

            // TODO: How to account for user-defined sizes, positions, etc?
            $(
                let mut component = $x.as_component();

                match &mut component {
                    WidgetOrView::Widget(widget) => {
                        widget.assign_id(current_id);
                        // TODO: In order to place text, we must first know its width
                        widget.place(default_padding, current_y + default_padding);

                        current_y += widget.rect().height() as i32 + default_padding;

                    }
                    WidgetOrView::View(subview) => {
                    }
                }

                view.push(component);

                current_id += 1;
            )+

            for item in &view {
                match item {
                    RustUI::view::WidgetOrView::Widget(widget) => {
                        let required_x = widget.rect().x() as u32 + widget.rect().width() as u32;
                        if required_x > max_x {
                            max_x = required_x;
                        }
                    }
                    RustUI::view::WidgetOrView::View(view) => {}
                }
            }

            let mut compiled_view = View {
                components: view,
                component_map: std::collections::HashMap::new(),
                view_width: max_x + default_padding as u32,
                view_height: current_y as u32 + default_padding as u32,
                fixed_size: false,
                default_padding: default_padding as u32,
                alignment: Alignment::Left,
            };

            // for item in &mut compiled_view.subview.components {
            //     match item {
            //         RustUI::view::WidgetOrView::Widget(widget) => {
            //             // TODO: Hash widgets
            //             // compiled_view.component_map.insert(widget.id(), widget);
            //         }
            //         RustUI::view::WidgetOrView::View(view) => {}
            //     }
            // }

            compiled_view
        }
    };
}

// ========================== HStack macro ========================== //

#[macro_export]
macro_rules! HStack {
    ( $($x:expr), + ) => {
        {
            let mut view = Vec::new();

            let default_padding = 10;
            // Current draw location
            let mut current_x = 0;

            let mut max_y: u32 = 0;

            // FIXME: This should be handled by Widget::new(str) and derive from the string
            let mut current_id = 10;

            // TODO: How to account for user-defined sizes, positions, etc?
            $(
                let mut component = $x.as_component();

                match &mut component {
                    WidgetOrView::Widget(widget) => {
                        widget.assign_id(current_id);
                        widget.place(current_x + default_padding, default_padding);

                        current_x += widget.rect().width() as i32 + default_padding;
                    }
                    WidgetOrView::View(subview) => {}
                }

                view.push(component);

                current_id += 1;
            )+

            for item in &view {
                match item {
                    RustUI::view::WidgetOrView::Widget(widget) => {
                        let required_y = widget.rect().y() as u32 + widget.rect().height() as u32;
                        if required_y > max_y {
                            max_y = required_y;
                        }
                    }
                    RustUI::view::WidgetOrView::View(view) => {}
                }
            }

            let mut compiled_view = View {
                components: view,
                component_map: std::collections::HashMap::new(),
                view_width: current_x as u32 + default_padding as u32,
                view_height: max_y + default_padding as u32,
                fixed_size: false,
                default_padding: default_padding as u32,
                alignment: Alignment::Left,
            };

            // for item in &mut compiled_view.subview.components {
            //     match item {
            //         RustUI::view::WidgetOrView::Widget(widget) => {
            //             // TODO: Hash widgets
            //             // compiled_view.component_map.insert(widget.id(), widget);
            //         }
            //         RustUI::view::WidgetOrView::View(view) => {}
            //     }
            // }

            compiled_view
        }
    };
}

// ========================== Example macro ========================== //

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
    ) => { // Contain the macro
        { // What the macro expands to
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
