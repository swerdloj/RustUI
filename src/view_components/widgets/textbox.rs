extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::view_components::{ViewComponent, WidgetOrView};
use crate::backend::system::window::Window;

use super::widget::{Widget, WidgetState, colors};
use super::Text;


/// TextBox widget
/// - Obtain user text input
pub struct TextBox<T> {
    id: &'static str,
    rect: Rect,
    background_color: Color,
    // hover_color: Color,
    active_color: Color,

    default_text: Option<Text<T>>,
    user_text: Option<Text<T>>,
}

impl<T> TextBox<T> {
    pub fn new(id: &'static str) -> Self {
        TextBox {
            id: id,
            rect: Rect::new(0, 0, 100, 40),
            background_color: colors::LIGHT_GRAY,
            // hover_color: 
            active_color: colors::WHITE,

            default_text: None,
            user_text: None,
        }
    }

    pub fn with_default_text(mut self, text: &str) -> Self {
        let mut owned_text = Text::new("", text)
            .with_color(colors::DARK_GRAY);

        // TODO: How to calculate this?
        // TODO: Should the size of this widget be based on text dimensions?
        owned_text.place(self.rect.x() + 5, self.rect.y);

        self.default_text = Some(owned_text);
        self
    }
}

impl<T> Widget<T> for TextBox<T> {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn primary_color(&self) -> Color {
        self.background_color
    }

    fn secondary_color(&self) -> Color {
        self.active_color
    }

    fn hover_color(&self) -> Color {
        self.background_color
    }

    fn text_component(&mut self) -> Option<&mut Text<T>> {
        // FIXME: Clean this up

        // Prioritize user text
        if let Some(text) = &mut self.user_text {
            return Some(text);
        }
        if let Some(text) = &mut self.default_text {
            return Some(text);
        }
        None
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        self.text_component().expect("Attempted to size nonexistant text component").assign_text_dimensions(dims);
    }

    fn render(&self, window: &mut Window<T>, widget_state: WidgetState) 
    where T: super::GenerateView<T, T> {
        match widget_state {
            WidgetState::Active => window.canvas.set_draw_color(self.active_color),
            _ => window.canvas.set_draw_color(self.background_color),
        }

        // Draw the background
        window.canvas.fill_rect(self.rect).unwrap();
        
        // Draw user_text > default_text
        if let Some(text) = &self.user_text {
            text.render(window, widget_state);
        }
        else if let Some(text) = &self.default_text {
            text.render(window, widget_state);
        }
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.rect = Rect::new(
            self.rect.x + dx,
            self.rect.y + dy,
            self.rect.width(),
            self.rect.height(),
        );

        if let Some(text) = &mut self.default_text {
            text.translate(dx, dy);
        }
        if let Some(text) = &mut self.user_text {
            text.translate(dx, dy);
        }
    }

    fn place(&mut self, x: i32, y: i32) {
        self.rect = Rect::new(x, y, self.rect.width(), self.rect.height());

        if let Some(text) = &mut self.default_text {
            text.container_rect = self.rect;
        }
        if let Some(text) = &mut self.user_text {
            text.container_rect = self.rect;
        }
    }

    fn draw_width(&self) -> u32 {
        self.rect.width()
    }

    fn draw_height(&self) -> u32 {
        self.rect.height()
    }

    fn on_click(&mut self, state: &mut T) {

    }

    fn update(&mut self, state: &T) {

    }
}

impl<T> ViewComponent<T> for TextBox<T> where T: 'static {
    fn as_component(self) -> WidgetOrView<T> {
        WidgetOrView::Widget(Box::new(self))
    }
}