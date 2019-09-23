// TODO: Replace VStack! macro to use this file (ensure everything works)
extern crate sdl2;
use sdl2::ttf;

use crate::font::{Fonts};

use crate::view_components::{WidgetOrView, ViewComponent, Padding};
use crate::view_components::widgets::widget::Widget;
use crate::view_components::views::view::{View, ViewData, Alignment};


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

impl<T> View<T> for VStack<T> {
    fn init(&mut self, ttf_context: &ttf::Sdl2TtfContext) {
        // TODO: How to extend this lifetime and implement for text rendering?
        let mut font_manager = Fonts::new();

        // Step 1 -> Initially size text components
        for item in &mut self.data.components {
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

        // Assign view dimensions if not defined
        if self.data.view_width == 0 {
            self.data.view_width = self.draw_width();
        }
        if self.data.view_height == 0 {
            self.data.view_height = self.draw_height();
        }

        //self.align(self.data.alignment.clone());
    }

    fn view_size(&self) -> (u32, u32) {
        (self.data.view_width, self.data.view_height)
    }

    fn widgets_mut(&mut self) -> Vec<&mut Box<dyn Widget<T>>> {
        let mut widgets = Vec::new();

        for component in &mut self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    widgets.push(widget);
                }
                _ => {}
            }
        }

        widgets
    }

    fn widgets(&self) -> Vec<&Box<dyn Widget<T>>> {
        let mut widgets = Vec::new();

        for component in &self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    widgets.push(widget);
                }
                _ => {}
            }
        }

        widgets
    }

    fn child_widgets(&mut self) -> Vec<&mut Box<dyn Widget<T>>> {
        let mut widgets = Vec::new();

        for component in &mut self.data.components {
            match component {
                WidgetOrView::Widget(widget) => {
                    widgets.push(widget);
                }
                WidgetOrView::View(subview) => {
                    widgets.append(&mut subview.child_widgets());
                }
            }
        }

        widgets
    }

    fn alignment(mut self, alignment: Alignment) -> Self {
        self.data.alignment = alignment;
        self
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
                    view.translate(dx, dy);
                }
            }
        }
    }

    fn align(&mut self) {
        let width = self.data.view_width;
        let alignment = self.data.alignment;

        match alignment {
            // Translate each widget to the center of the view
            Alignment::Center => {
                for component in &mut self.data.components {
                    match component {
                        // Center widget within view
                        WidgetOrView::Widget(widget) => {
                            let new_x = (width / 2) as i32 - (widget.draw_width() / 2) as i32;
                            widget.translate(new_x - widget.rect().x(), 0);
                        }
                        // Shift subview to center of current view
                        WidgetOrView::View(subview) => {
                            // FIXME: Padding? Why the -5? Half padding?
                            let shift_x = (width / 2) as i32 - (subview.draw_width() / 2) as i32;
                            subview.translate(shift_x, 0);
                        }
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

                WidgetOrView::View(subview) => {
                    if subview.draw_width() > max_width {
                        max_width = subview.draw_width();
                    }
                }
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
                WidgetOrView::View(subview) => {
                    height += subview.draw_height();
                }
            }
        }

        height + self.data.padding.top + self.data.padding.bottom
    }
}

impl<T> ViewComponent<T> for VStack<T> where T: 'static{
    fn as_component(self) -> WidgetOrView<T> {
        WidgetOrView::View(Box::new(self))
    }
}

#[macro_export]
macro_rules! VStack2 {
    ( $($x:expr), + ) => {
        {
            let mut components = Vec::new();
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

                        current_y += widget.draw_height() as i32 + default_padding;
                    }

                    WidgetOrView::View(subview) => {
                        subview.translate(0, current_y);
                        
                        current_y += subview.draw_height() as i32 + default_padding;
                    }
                }

                components.push(component);
            )+

            VStack::new(components)
        }
    };
}