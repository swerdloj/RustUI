extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;

use crate::view_components::{ViewComponent, IntoViewComponent};
use crate::backend::system::window::Window;
use crate::colors;

use super::widget::{Widget, WidgetState};
use super::Text;

pub struct ScrollBar<T> {
    id: &'static str,

    from: i32,
    to: i32,
    value: i32,
    
    slider: Rect,
    rail: Rect,

    value_text: Option<Text<T>>,

    on_value_changed: Option<Box<dyn Fn(&mut T, i32)>>,

    slider_passive_color: Color,
    slider_hover_color: Color,
    slider_active_color: Color,
    rail_passive_color: Color,
    rail_hover_color: Color,
}

impl<T> ScrollBar<T> {
    pub fn new(id: &'static str, from: i32, to: i32, current: i32) -> Self {
        ScrollBar {
            id: id,

            from: from,
            to: to,
            value: current,

            slider: Rect::new(0, 0, 12, 16),
            rail: Rect::new(0, 0, 120, 4),

            value_text: None,

            on_value_changed: None,

            slider_passive_color: colors::MANILLA,
            slider_hover_color: Color::RGB(200, 200, 200),
            slider_active_color: colors::DARK_GRAY,
            rail_passive_color: colors::LIGHT_GRAY,
            rail_hover_color: colors::WHITE,
        }
    }

    pub fn with_length(mut self, length: u32) -> Self {
        self.rail.set_width(length);
        self
    }

    pub fn with_thickness(mut self, thickness: u32) -> Self {
        self.rail.set_height(thickness);
        self
    }

    pub fn with_on_value_changed<F: 'static + Fn(&mut T, i32)>
    (mut self, callback: F) -> Self {
        self.on_value_changed = Some(Box::new(callback));
        self
    }

    /// Maps slider location relative to the rail to slider value
    fn pixel_to_value(&self, pixel: i32) -> i32 {
        let value = ((pixel-self.rail.x) * (self.to - self.from)) / self.rail.width() as i32;
        
        return if value < self.from {
            self.from
        } else if value > self.to {
            self.to
        } else {
            value
        };
    }

    /// Maps slider value to slider location relative to the rail
    fn value_to_pixel(&self, value: i32) -> i32 {
        (value * self.rail.width() as i32) / (self.to - self.from) + self.rail.x
    }
}

impl<T> Widget<T> for ScrollBar<T> {
    // TODO: This
    fn rect(&self) -> Rect {
        self.slider
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn text_component(&mut self) -> Option<&mut Text<T>> {
        self.value_text.as_mut()
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        if let Some(text) = &mut self.value_text {
            text.assign_text_dimensions(dims);
        }
    }

    fn place(&mut self, x: i32, y: i32) {
        self.rail.set_x(x);
        self.rail.set_y(y);
        // self.slider.set_x(self.value_to_pixel(self.value) - self.slider.width() as i32 / 2);
        self.slider.set_y(y - (self.slider.height() - self.rail.height()) as i32 / 2);
    }

    fn update(&mut self, state: &mut T, event: &Event) {
        match event {
            Event::MouseMotion {x, y, ..} => {
                self.value = self.pixel_to_value(*x);
                self.slider.set_x(self.value_to_pixel(self.value) - self.slider.width() as i32 / 2);

                if let Some(on_value_changed) = &self.on_value_changed {
                    (on_value_changed)(state, self.value);
                }
            }
            _ => {}
        }
    }

    fn render(&self, window: &mut Window<T>, widget_state: WidgetState)
    where T: super::GenerateView<T> {
        // Draw rail
        match widget_state {
            // WidgetState::Active |
            // WidgetState::Hovering => window.canvas.set_draw_color(self.rail_hover_color),
            _ => window.canvas.set_draw_color(self.rail_passive_color),
        }
        window.canvas.fill_rect(self.rail).unwrap();

        // Draw slider
        match widget_state {
            WidgetState::Active => window.canvas.set_draw_color(self.slider_active_color),
            WidgetState::Hovering => window.canvas.set_draw_color(self.slider_hover_color),
            _ => window.canvas.set_draw_color(self.slider_passive_color),
        }
        window.canvas.fill_rect(self.slider).unwrap();
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.rail.set_x(self.rail.x() + dx);
        self.rail.set_y(self.rail.y() + dy);
        // FIXME: This is an alignment hack. This should actually be done in .place,
        //  but because view.align uses widget.rect(), moving this to .place
        //  will only adjust the rail, not the slider.
        //  This is because .rect() returns the slider's rect (for mouse handling), not the 
        //  rail's rect
        self.slider.set_x(self.value_to_pixel(self.value) - self.slider.width() as i32 / 2);
        self.slider.set_y(self.slider.y() + dy);
    }

    fn draw_width(&self) -> u32 {
        self.rail.width()
    }

    fn draw_height(&self) -> u32 {
        self.slider.height()
    }
}

impl<T> IntoViewComponent<T> for ScrollBar<T> where T: 'static {
    fn as_component(self) -> ViewComponent<T> {
        ViewComponent::Widget(Box::new(self))
    }
}