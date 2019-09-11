// TODO: Replace VStack! macro to use this file (ensure everything works)

use super::view::*;
use crate::view::{Alignment, WidgetOrView};

pub struct VStack<T> {
    data: ViewData<T>,
}

impl<T> VStack<T> {
    pub fn new(components: Vec<WidgetOrView<T>>) -> Self {
        VStack {
            data:
                ViewData {
                    component_map: std::collections::HashMap::new(),
                    components: components,
                    view_width: 0,
                    view_height: 0,
                    fixed_size: false,
                    padding:
                        Padding {
                            left: 10,
                            right: 10,
                            top: 10,
                            bottom: 10,
                        },
                    alignment: Alignment::Center,
                }
        }
    }
}

impl<T> View for VStack<T> {
    fn init(&mut self) {

    }

    fn fixed_width(mut self, width: u32) -> Self {
        self.data.view_width = width;
        self
    }

    fn fixed_height(mut self, height: u32) -> Self {
        self.data.view_height = height;
        self
    }

    fn fixed_size(mut self, width: u32, height: u32) -> Self {
        self.data.view_width = width;
        self.data.view_height = height;
        self
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        // Translate all components by the same amound
        for component in &mut self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    widget.translate(dx, dy);
                }
                WidgetOrView::View(view) => {
                    // FIXME:
                    // view.translate(dx, dy);
                }
            }
        }
    }

    fn align(&mut self, alignment: Alignment) {
        let draw_width = self.draw_width();

        match alignment {
            // Translate each widget to the center of the view
            Alignment::Center => {
                for component in &mut self.data.components {
                    match component {
                        WidgetOrView::Widget(widget) => {
                            let new_x = (draw_width / 2) as i32 - (widget.draw_width() / 2) as i32;
                            widget.translate(new_x - widget.rect().x(), 0);
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                // TODO: Implement the rest
            }
        }
    }

    fn draw_width(&self) -> u32 {
        let mut max_width = 0u32;

        for component in &self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    if widget.draw_width() > max_width {
                        max_width = widget.draw_width();
                    }
                }
                _ => {}
            }
        }

        max_width + self.data.padding.right + self.data.padding.left
    }

    fn draw_height(&self) -> u32 {
        let mut height = 0u32;

        for component in &self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    height += widget.draw_height();
                }
                _ => {}
            }
        }

        height + self.data.padding.top + self.data.padding.bottom
    }
}

macro_rules! VStack {
    ( $($x:expr), + ) => {
        {
            let mut components: Vec<WidgetOrView<T>> = Vec::new();
            // let mut vstack = VStack::new(components);
            let default_padding = 10;

            let mut current_y = 0;

            // FIXME: Replace this with string in widget.rs
            let mut current_id = 0;
            
            $(
                let mut component = $x.as_component();

                match &mut component {
                    // FIXME: Placement needs to occur in the init function
                    WidgetOrView::Widget(widget) => {
                        widget.assign_id(current_id);
                        current_id += 1;

                        // TODO: Account for padding here?
                        widget.place(0, current_y);

                        current_y += widget.rect().height();
                    }

                    WidgetOrView::VIew(subview) => {
                        for widget in subview.widgets_mut() {
                            widget.translate(0, current_y);
                        }
                        current_y += subview.draw_height() as i32;
                    }
                }

                components.push(component);
                
            )+
        }

        VStack::new(components)
    };
}