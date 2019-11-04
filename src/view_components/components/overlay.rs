extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use crate::Window;

use crate::view_components::views::view::View;

use super::Component;
use super::super::{IntoViewComponent, ViewComponent};

/* TODO:

There is no way to deal with overlays or similar until the font backend
is sorted out. Ideally, the font cache will be part of the Window struct,
and will be needed to render fonts properly here.

Furthermore, the backend does not currently have the capability to handle
anything other than widgets, meaning `Overlay` does not receive events and
is never updated

*/

pub struct Overlay<T> {
    /// Overlay color includes transparency
    overlay_color: Color,
    overlay_view: Box<dyn View<T>>,
}

impl<T> Overlay<T> {
    pub fn new<V: View<T> + 'static>
    (overlay_view: V) -> Self {
        Overlay {
            overlay_color: Color::RGBA(0, 0, 0, 180),
            overlay_view: Box::new(overlay_view),
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.overlay_color = color;
        self
    }
}

impl<T> Component<T> for Overlay<T> {
    fn place(&mut self, x: i32, y: i32) {
        // Overlays will always be centered within the screen
    }

    fn render(&self, window: &mut Window<T>, parent_dimensions: (u32, u32))
    where T: crate::state::GenerateView<T> {
        let (width, height) = window.canvas.output_size().expect("Failed to query canvas size");
        
        // Draw the overlay
        window.canvas.set_blend_mode(BlendMode::Blend);
        window.canvas.set_draw_color(self.overlay_color);
        window.canvas.fill_rect(Rect::new(0, 0, width, height));
        // Return to default blend mode
        window.canvas.set_blend_mode(BlendMode::None);

        for widget in self.overlay_view.widgets() {
            widget.render(window, crate::widgets::widget::WidgetState::Base);
        }
    }

    fn draw_width(&self) -> u32 {
        0
    }

    fn draw_height(&self) -> u32 {
        0
    }
}

impl<T> IntoViewComponent<T> for Overlay<T> where T: 'static {
    fn as_component(self) -> ViewComponent<T> {
        ViewComponent::Component(Box::new(self))
    }
}