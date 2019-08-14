extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::view::{ViewComponent, WidgetOrView};
use crate::backend::system::window::Window;

use super::widget::{Widget, WidgetState, colors};
use super::text::Text;


pub struct Button<T> {
    pub id: u32,
    pub rect: Rect,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub hover_color: Color,
    pub text: Option<Text<T>>,
    pub on_click: Option<Box<Fn(&mut T)>>,
}

impl<T> Button<T> {
    pub fn new(id: &str) -> Self {
        Button {
            id: 0,
            rect: Rect::new(0, 0, 100, 40),
            primary_color: colors::MANILLA,
            secondary_color: Color::RGB(100, 100, 100),
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
}

impl<T> Widget<T> for Button<T> {
    fn assign_id(&mut self, id: u32) {
        self.id = id;
    }

    fn place(&mut self, x: i32, y: i32)  {
        // Place the button at (x, y)
        self.rect = Rect::new(x, y, self.rect.width(), self.rect.height());

        // Place the button text respective of this new position
        if let Some(button_text) = &mut self.text {
            button_text.container_rect = self.rect;
        }
    }

    fn text_component(&mut self) -> Option<&mut Text<T>> {
        if let Some(text) = &mut self.text {
            return Some(text);
        }
        None
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        self.text_component().expect("Attempted to size nonexistant text component").assign_text_dimensions(dims);
    }

    fn render(&self, window: &mut Window<T>, widget_state: WidgetState) {
        match widget_state {
            WidgetState::Hovering => window.canvas.set_draw_color(self.hover_color),
            WidgetState::Active => window.canvas.set_draw_color(self.secondary_color),
            _ => window.canvas.set_draw_color(self.primary_color),
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

    fn id(&self) -> u32 {
        self.id
    }

    fn primary_color(&self) -> Color {
        self.primary_color
    }

    fn secondary_color(&self) -> Color {
        self.secondary_color
    }

    fn hover_color(&self) -> Color {
        self.hover_color
    }

    fn update(&mut self, state: &T) {

    }

    fn on_click(&mut self, state: &mut T) {
        if let Some(ref on_click_function) = self.on_click {
            (on_click_function)(state);
        }
    }

    fn draw_width(&self) -> u32 {
        self.rect.width()
    }
}

impl<T> ViewComponent<T> for Button<T> where T: 'static {
    fn as_component(self) -> WidgetOrView<T> {
        WidgetOrView::Widget(Box::new(self))
    }
}