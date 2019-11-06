extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::ttf;

use crate::Window;
use crate::View;
use crate::Widget;
use crate::Alignment;
use crate::Component;

use crate::font::Fonts;

use crate::widgets::Button;

use super::view::ViewData;
use super::super::{IntoViewComponent, ViewComponent};

/* TODO:

There is no way to deal with overlays or similar until the font backend
is sorted out. Ideally, the font cache will be part of the Window struct,
and will be needed to render fonts properly here.

Furthermore, the backend does not currently have the capability to handle
anything other than widgets, meaning `Overlay` does not receive events and
is never updated

SOLUTION: Make overlay a view type?

*/

pub struct Overlay<T> {
    /// Overlay color includes transparency
    overlay_color: Color,
    
    data: ViewData<T>,
}

impl<T> Overlay<T> {
    pub fn new(mut components: Vec<ViewComponent<T>>) -> Self 
    where T: 'static {
        components.push(Button::new("__overlayButton")
                            // .with_dimensions(width: u32, height: u32)
                            // .color..
                            .with_dimensions(0, 0)
                            .as_component()
        );

        Overlay {
            overlay_color: Color::RGBA(0, 0, 0, 180),
            data: ViewData {
                component_map: std::collections::HashMap::new(),
                components: components,
                view_width: 0,
                view_height: 0,
                fixed_size: false,
                alignment: Alignment::Center,
            }
        }
    }

    pub fn with_overlay_color(mut self, color: Color) -> Self {
        self.overlay_color = color;
        self
    }
}
// fn render(&self, window: &mut Window<T>, parent_dimensions: (u32, u32))
// where T: crate::state::GenerateView<T> {
//     let (width, height) = window.canvas.output_size().expect("Failed to query canvas size");
    
//     // Draw the overlay
//     window.canvas.set_blend_mode(BlendMode::Blend);
//     window.canvas.set_draw_color(self.overlay_color);
//     window.canvas.fill_rect(Rect::new(0, 0, width, height));
//     // Return to default blend mode
//     window.canvas.set_blend_mode(BlendMode::None);

//     for widget in self.overlay_view.widgets() {
//         widget.render(window, crate::widgets::widget::WidgetState::Base);
//     }
// }

impl<T> View<T> for Overlay<T> {
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
    }

    fn translate(&mut self, dx: i32, dy: i32) {
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

        // Always cenetered within the screen
        for component in &mut self.data.components {
            match component {
                ViewComponent::Widget(widget) => {
                    let new_x = (width / 2) as i32 - (widget.draw_width() / 2) as i32;
                    widget.translate(new_x - widget.rect().x(), 0);
                }
                ViewComponent::View(subview) => {
                    let shift_x = (width / 2) as i32 - (subview.draw_width() / 2) as i32;
                    subview.translate(shift_x, 0);
                }
                _ => {}
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

        max_width;

        0
    }

    fn draw_height(&self) -> u32 {
        let mut max_height = 0u32;

        for component in &self.data.components {
            match component {
                ViewComponent::Widget(widget) => {
                    // FIXME: Casting y coord to u32 is bad
                    if widget.draw_height() + widget.rect().y() as u32 > max_height {
                        max_height = widget.draw_height() + widget.rect().y() as u32;
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

        max_height;

        0
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
    
    fn overlay(&mut self, overlay: Overlay<T>) where T: 'static {
        panic!("Cannot overlay an overlay");
    }

    fn alignment(mut self, alignment: Alignment) -> Self {
        self
    }

    fn fixed_width(mut self, width: u32) -> Self {
        self
    }

    fn fixed_height(mut self, height: u32) -> Self {
        self
    }

    fn fixed_size(mut self, width: u32, height: u32) -> Self {
        self
    }

    fn padding(mut self, left: u32, right: u32, top: u32, bottom: u32) -> Self {
        self
    }


}

impl<T> IntoViewComponent<T> for Overlay<T> where T: 'static {
    fn as_component(self) -> ViewComponent<T> {
        ViewComponent::View(Box::new(self))
    }
}

#[macro_export]
macro_rules! VOverlay {
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

            Overlay::new(components)
        }
    };
}