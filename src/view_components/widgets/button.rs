extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::view_components::{ViewComponent, IntoViewComponent};
use crate::backend::system::window::Window;
use crate::colors;

use super::widget::{Widget, WidgetState};
use super::text::Text;

pub struct Button<T> {
    pub id: &'static str,
    pub rect: Rect,
    pub passive_color: Color,
    pub clicking_color: Color,
    pub hover_color: Color,
    pub text: Option<Text<T>>,
    pub on_click: Option<Box<dyn Fn(&mut T)>>,
}

impl<T> Button<T> {
    pub fn new(id: &'static str) -> Self {
        Button {
            id: id,
            rect: Rect::new(0, 0, 100, 40),
            passive_color: colors::MANILLA,
            clicking_color: Color::RGB(100, 100, 100),
            hover_color: Color::RGB(200, 200, 200),
            text: None,
            on_click: None,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        // TODO: How to hanle the sub-widget's id?
        //       Note that the sub-widget is not actually part of the view
        self.text = Some(
            Text::new("", text)
            .center()
        );
        self
    }

    pub fn with_on_click<F: 'static + Fn(&mut T)>
    (mut self, callback: F) -> Self
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    /// Assign specific width to button
    pub fn with_width(mut self, width: u32) -> Self {
        self.rect.set_width(width);
        self
    }

    /// Assign specific height to button
    pub fn with_height(mut self, height: u32) -> Self {
        self.rect.set_height(height);
        self
    }

    /// Assign width and height to button
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.rect.set_width(width);
        self.rect.set_height(height);
        self
    }

    /// Assign base button color
    pub fn with_base_color(mut self, color: Color) -> Self {
        self.passive_color = color;
        self
    }

    /// Assign color on hover
    pub fn with_hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    /// Assign click color
    pub fn with_click_color(mut self, color: Color) -> Self {
        self.clicking_color = color;
        self
    }
}

impl<T> Widget<T> for Button<T> {
    fn place(&mut self, x: i32, y: i32)  {
        // Place the button at (x, y)
        self.rect = Rect::new(x, y, self.rect.width(), self.rect.height());

        // Place the button text respective of this new position
        if let Some(button_text) = &mut self.text {
            button_text.container_rect = self.rect;
        }
    }

    fn text_component(&mut self) -> Option<&mut Text<T>> {
        self.text.as_mut()
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        self.text_component().expect("Attempted to size nonexistant text component").assign_text_dimensions(dims);
    }

    fn render(&self, window: &mut Window<T>, widget_state: WidgetState)
    where T: super::GenerateView<T> {
        match widget_state {
            WidgetState::Hovering => window.canvas.set_draw_color(self.hover_color),
            WidgetState::Active => window.canvas.set_draw_color(self.clicking_color),
            _ => window.canvas.set_draw_color(self.passive_color),
        }

        window.canvas.fill_rect(self.rect).unwrap();
        // pay attention to draw order
        if let Some(button_text) = &self.text {
            button_text.render(window, widget_state);
        }
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.rect = Rect::new(
            self.rect().x() + dx,
            self.rect().y() + dy,
            self.rect().width(),
            self.rect().height()
        );

        if let Some(button_text) = &mut self.text {
            button_text.translate(dx, dy);
        }
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn on_click(&mut self, state: &mut T) {
        if let Some(ref on_click_function) = self.on_click {
            (on_click_function)(state);
        }
    }

    fn draw_width(&self) -> u32 {
        self.rect.width()
    }

    fn draw_height(&self) -> u32 {
        self.rect.height()
    }
}

impl<T> IntoViewComponent<T> for Button<T> where T: 'static {
    fn as_component(self) -> ViewComponent<T> {
        ViewComponent::Widget(Box::new(self))
    }
}