// TODO: Replace VStack! macro to use this file (ensure everything works)
extern crate sdl2;
use sdl2::ttf;

use crate::font::{Fonts};

use crate::view_components::{ViewComponent, IntoViewComponent, Padding};
use crate::view_components::widgets::widget::Widget;
use crate::view_components::components::Component;
use crate::view_components::views::view::{View, ViewData, Alignment};


pub struct VStack<T> {
    data: ViewData<T>,
    padding: Padding,
}

impl<T> VStack<T> {
    pub fn new(components: Vec<ViewComponent<T>>) -> Self {
        VStack {
            data:
                ViewData {
                    component_map: std::collections::HashMap::new(),
                    components: components,
                    view_width: 0,
                    view_height: 0,
                    fixed_size: false,
                    alignment: Alignment::Center,
                },
            padding:
                Padding {
                    left: 10,
                    right: 10,
                    top: 10,
                    bottom: 10,
                },
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

    fn overlay(&mut self, overlay: super::Overlay<T>) where T: 'static{
        self.data.components.push(overlay.as_component());
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

    fn child_comps(&self) -> Vec<&Box<dyn Component<T>>> {
        let mut comps = Vec::new();

        for component in &self.data.components {
            match component {
                ViewComponent::Component(comp) => {
                    comps.push(comp);
                }
                ViewComponent::View(subview) => {
                    comps.append(&mut subview.child_comps());
                }
                _ => {}
            }
        }

        comps
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
        self.padding.left = left;
        self.padding.right = right;
        self.padding.top = top;
        self.padding.bottom = bottom;
        self
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        // Translate all components by the same amound
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
        let width = self.data.view_width;
        let alignment = self.data.alignment;

        match alignment {
            // Translate each widget to the center of the view
            Alignment::Center => {
                for component in &mut self.data.components {
                    match component {
                        // Center widget within view
                        ViewComponent::Widget(widget) => {
                            let new_x = (width / 2) as i32 - (widget.draw_width() / 2) as i32;
                            widget.translate(new_x - widget.rect().x(), 0);
                        }
                        // Shift subview to center of current view
                        ViewComponent::View(subview) => {
                            let shift_x = (width / 2) as i32 - (subview.draw_width() / 2) as i32;
                            subview.translate(shift_x, 0);
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
                ViewComponent::Widget(widget) => {
                    if widget.draw_width() > max_width {
                        max_width = widget.draw_width();
                    }
                }

                ViewComponent::View(subview) => {
                    if subview.draw_width() > max_width {
                        max_width = subview.draw_width();
                    }
                }
                _ => {}
            }
        }

        max_width + self.padding.right + self.padding.left
    }

    fn draw_height(&self) -> u32 {
        let mut max_height = 0u32;
        let mut current_height: u32;

        for component in &self.data.components {
            match component {
                ViewComponent::Widget(widget) => {
                    current_height = widget.rect().y as u32 + widget.draw_height();
                    if current_height > max_height {
                        max_height = current_height;
                    }
                }
                ViewComponent::View(subview) => {
                    // FIXME: Account for view's y position
                    //  The below line is a (working/logical) hack
                    //  This only works because VStacks increase height with each widget
                    current_height = max_height + subview.draw_height();
                    if current_height > max_height {
                        max_height = current_height;
                    }
                }
                _ => {}
            }
        }

        max_height + self.padding.top + self.padding.bottom
    }
}

impl<T> IntoViewComponent<T> for VStack<T> where T: 'static {
    fn as_component(self) -> ViewComponent<T> {
        ViewComponent::View(Box::new(self))
    }
}

#[macro_export]
macro_rules! VStack {
    ( $($x:expr), + ) => {
        {
            let mut components = Vec::new();
            // let mut vstack = VStack::new(components);
            let default_padding = 10;

            let mut current_y = default_padding;
            
            $(
                let mut component = $x.as_component();

                match &mut component {
                    // FIXME: Placement needs to occur in the init function
                    ViewComponent::Widget(widget) => {
                        // TODO: Account for padding here?
                        widget.place(0, current_y);

                        current_y += widget.draw_height() as i32 + default_padding;
                    }

                    ViewComponent::View(subview) => {
                        subview.translate(0, current_y);
                        
                        current_y += subview.draw_height() as i32 + default_padding;
                    }

                    ViewComponent::Component(comp) => {
                        comp.place(0, current_y);
                        current_y += comp.draw_height() as i32 + default_padding;
                    }
                }

                components.push(component);
            )+

            VStack::new(components)
        }
    };
}