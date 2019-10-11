extern crate sdl2;
use sdl2::ttf;

use crate::font::Fonts;

use crate::view_components::{ViewComponent, IntoViewComponent, Padding};
use crate::view_components::widgets::widget::Widget;
use crate::view_components::views::view::{View, ViewData, Alignment};


pub struct HStack<T> {
    data: ViewData<T>,
}

impl<T> HStack<T> {
    pub fn new(components: Vec<ViewComponent<T>>) -> Self {
        HStack {
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

impl<T> View<T> for HStack<T> {
    fn init(&mut self, ttf_context: &ttf::Sdl2TtfContext) {
        // TODO: How to extend this lifetime and implement for text rendering?
        let mut font_manager = Fonts::new();

        // Initially size text components
        for item in &mut self.data.components {
            match item {
                ViewComponent::Widget(widget) => {
                    // If the widget has a text component, obtain its surface size
                    if let Some(text_component) = widget.text_component() {
                        font_manager.load_font(ttf_context, &text_component.font);
                        let text_surface_size = font_manager.size_surface(&text_component.font, &text_component.text);
                        widget.assign_text_dimensions(text_surface_size);
                    }
                }

                ViewComponent::View(nested_view) => {
                    nested_view.init(ttf_context);
                }

                _ => {}
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
                ViewComponent::Widget(widget) => {
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
                ViewComponent::Widget(widget) => {
                    widgets.push(widget);
                }
                _ => {}
            }
        }

        widgets
    }

    fn child_widgets_mut(&mut self) -> Vec<&mut Box<dyn Widget<T>>> {
        let mut widgets = Vec::new();

        for component in &mut self.data.components {
            match component {
                ViewComponent::Widget(widget) => {
                    widgets.push(widget);
                }
                ViewComponent::View(subview) => {
                    widgets.append(&mut subview.child_widgets_mut());
                }
                _ => {}
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

    fn padding(mut self, left: u32, right: u32, top: u32, bottom: u32) -> Self {
        self.data.padding.left = left;
        self.data.padding.right = right;
        self.data.padding.top = top;
        self.data.padding.bottom = bottom;
        self
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        // Translate all components by the same amount
        for component in &mut self.data.components {
            match component {
                ViewComponent::Widget(widget) => {
                    widget.translate(dx, dy);
                }

                ViewComponent::View(view) => {
                    view.translate(dx, dy);
                }
                _ => {}
            }
        }
    }

    fn align(&mut self) {
        // let draw_width = self.draw_width();
        let alignment = self.data.alignment;

        match alignment {
            Alignment::Center => {
                // Distance from HStack's center to the view center
                let x_offset = (self.data.view_width / 2) - (self.draw_width() / 2);

                for component in &mut self.data.components {
                    match component {
                        ViewComponent::Widget(widget) => {
                            widget.translate(x_offset as i32, 0);
                        }
                        ViewComponent::View(subview) => {
                            // FIXME: Confirm this is correct
                            subview.translate(x_offset as i32, 0);
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                // TODO: Implement the rest
                //  Alignment::Left should be default (no extra work)
            }
        }
    }

    fn draw_width(&self) -> u32 {
        // FIXME: We fill this recursively because in the event
        //        a nested view has other, wider nested views,
        //        we want to guarentee that we avoid overlapping widgets
        //        caused by wide, nested views

        // FIXME: Confirm this
        let mut draw_width = 0;

        let mut rightmost_x = 0;
        let mut leftmost_x = self.data.view_width as i32;

        for component in &self.data.components {
            match component {
                ViewComponent::Widget(widget) => {
                    if widget.rect().x() < leftmost_x {
                        leftmost_x = widget.rect().x();
                    }

                    if widget.rect().x() > rightmost_x {
                        rightmost_x = widget.rect().x() + widget.draw_width() as i32;
                    }
                }
                ViewComponent::View(subview) => {
                    draw_width += subview.draw_width();
                }
                _ => {}
            }
        }

        // leftmost_x > rightmost_x
        draw_width + (rightmost_x - leftmost_x) as u32 + self.data.padding.left + self.data.padding.right
    }

    fn draw_height(&self) -> u32 {
        let mut max_height = 0u32;

        for component in &self.data.components {
            match component {
                ViewComponent::Widget(widget) => {
                    if widget.draw_height() > max_height {
                        max_height = widget.draw_height();
                    }
                }
                ViewComponent::View(subview) => {
                    if subview.draw_height() > max_height {
                        max_height = subview.draw_height();
                    }
                }
                _ => {}
            }
        }

        max_height + self.data.padding.top + self.data.padding.bottom
    }
}

impl<T> IntoViewComponent<T> for HStack<T> where T: 'static {
    fn as_component(self) -> ViewComponent<T> {
        ViewComponent::View(Box::new(self))
    }
}

#[macro_export]
macro_rules! HStack {
    ( $($x:expr), + ) => {
        {
            let mut components = Vec::new();

            let default_padding = 10;

            let mut current_x = default_padding;
            
            $(
                let mut component = $x.as_component();

                match &mut component {
                    // FIXME: Placement needs to occur in the init function
                    ViewComponent::Widget(widget) => {
                        widget.place(current_x, 0);

                        current_x += widget.draw_width() as i32 + default_padding;
                    }

                    ViewComponent::View(subview) => {
                        subview.translate(current_x, 0);
                        
                        current_x += subview.draw_width() as i32 + default_padding;
                    }
                    _ => {}
                }

                components.push(component);                
            )+

            HStack::new(components)
        }
    };
}