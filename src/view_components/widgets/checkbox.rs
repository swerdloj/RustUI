extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::view_components::{ViewComponent, WidgetOrView};
use crate::backend::system::window::Window;

use super::widget::{Widget, WidgetState, colors};
use super::text::Text;


pub struct CheckBox<T> {
    id: &'static str,
    rect: Rect,
    default_color: Color,
    click_color: Color,
    hover_color: Color,
    check_color: Color,
    text: Option<Text<T>>,
    is_checked: bool,
    
    checkbox_padding_right: u32,
    // Callback accepting application state & check state
    on_check: Option<Box<dyn Fn(&mut T, bool)>>,

    checkbox_width: u32,
    checkbox_height: u32,
}

impl<T> CheckBox<T> {
    pub fn new(id: &'static str, is_checked: bool) -> Self {
        CheckBox {
            id: id,
            rect: Rect::new(0, 0, 100, 40),
            default_color: colors::MANILLA,
            click_color: Color::RGB(140, 140, 140),
            hover_color: Color::RGB(200, 200, 200),
            check_color: Color::RGB(80, 80, 80),
            text: None,
            is_checked: is_checked,
            checkbox_padding_right: 10,
            on_check: None,

            checkbox_width: 20,
            checkbox_height: 20,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        // TODO: How to hanle the sub-widget's id?
        //       Note that the sub-widget is not actually part of the view

        let mut attached_text = Text::new("", text).with_color(colors::WHITE);
        attached_text.place(self.rect.x() + (self.checkbox_width + self.checkbox_padding_right) as i32, 
                       self.rect.y());

        self.text = Some(attached_text);
        self
    }

    pub fn with_on_check<F: 'static + Fn(&mut T, bool)>
    (mut self, check_fn: F) -> Self {
        self.on_check = Some(Box::new(check_fn));
        self
    }

    pub fn check(mut self) -> Self {
        self.is_checked = true;
        self
    }

    pub fn on_check(&mut self, state: &mut T) {
        self.is_checked = !self.is_checked;

        if let Some(on_check_fn) = &self.on_check {
            (on_check_fn)(state, self.is_checked);
        }
    }

}

impl<T> Widget<T> for CheckBox<T> {
    fn assign_id(&mut self, id: &'static str) {
        self.id = id;
    }

    fn place(&mut self, x: i32, y: i32) {
        self.rect = Rect::new(x, y, self.rect.width(), self.rect.height());

        if let Some(text) = &mut self.text {
            text.container_rect = Rect::new(
                x + (self.checkbox_width + self.checkbox_padding_right) as i32,
                // TODO: Is this y position correct?
                self.rect.y(),
                self.rect.width(),
                self.rect.height()
            );
        }
    }

    fn text_component(&mut self) -> Option<&mut Text<T>> {
        if let Some(text) = &mut self.text {
            return Some(text);
        }
        None
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        self.rect.set_width(self.checkbox_width + self.checkbox_padding_right + dims.0);
        self.text_component().expect("Attempted to size nonexistant text component").assign_text_dimensions(dims);
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn primary_color(&self) -> Color {
        self.default_color
    }

    fn secondary_color(&self) -> Color {
        self.click_color
    }

    fn hover_color(&self) -> Color {
        self.hover_color
    }

    // TODO: Allow user to decide whether this should trigger when the widget is clicked
    //       or *only* when the checkbox itself is clicked (would need x/y coords)
    fn on_click(&mut self, state: &mut T) {
        self.on_check(state);
    }

    // TODO: Should the text also change color on hover? such as making it slightly gray?
    fn render(&self, window: &mut Window<T>, widget_state: WidgetState)
    where T: super::GenerateView<T, T> {
        // First, draw the checkbox itself
        match widget_state {
            WidgetState::Hovering => window.canvas.set_draw_color(self.hover_color),
            WidgetState::Active => window.canvas.set_draw_color(self.click_color),
            WidgetState::Base => window.canvas.set_draw_color(self.default_color),
        }
        let checkbox_x = self.rect.x();// + self.internal_padding as i32;
        let checkbox_y = self.rect.y() + (self.rect.height() as i32 - self.checkbox_height as i32) / 2;

        window.canvas.fill_rect(Rect::new(checkbox_x, checkbox_y, self.checkbox_width, self.checkbox_height)).unwrap();
    
        // Second, draw the check if checked
        if self.is_checked {
            window.canvas.set_draw_color(self.check_color);
            window.canvas.fill_rect(Rect::new(
                checkbox_x + 4, 
                checkbox_y + 4, 
                self.checkbox_width - 8, 
                self.checkbox_height - 8)
            ).unwrap();
        }

        // Finally, draw the text if present
        if let Some(text) = &self.text {
            text.render(window, widget_state);
        }
    }

    fn update(&mut self, state: &T) {

    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.rect = Rect::new(
            self.rect.x() + dx,
            self.rect.y() + dy,
            self.rect.width(),
            self.rect.height()
        );

        if let Some(text) = &mut self.text {
            text.translate(dx, dy);
        }
    }

    fn draw_width(&self) -> u32 {
        self.rect().width()
    }

    fn draw_height(&self) -> u32 {
        self.rect().height()
    }
}

impl<T> ViewComponent<T> for CheckBox<T> where T: 'static {
    fn as_component(self) -> WidgetOrView<T> {
        WidgetOrView::Widget(Box::new(self))
    }
}