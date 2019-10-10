extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
    focus_color: Color,

    default_text: Text<T>,
    user_text: Text<T>,

    // FIXME: This is a hack
    //  update() is called before render(), so adding characters
    //  will display them for a single frame in wrong proportions.
    //  This is a workaround for holding multiple characters.
    input_buffer: String,

    // Interacts with user state when text input changes
    pub on_text_changed: Option<Box<dyn Fn(&mut T, String)>>,
    // Notifies that text entry is submitted (Enter key)
    pub on_text_submit: Option<Box<dyn Fn(&mut T, String)>>,
}

impl<T> TextBox<T> {
    pub fn new(id: &'static str, text: String) -> Self {
        TextBox {
            id: id,
            rect: Rect::new(0, 0, 100, 40),
            background_color: colors::LIGHT_GRAY,
            // hover_color: 
            focus_color: colors::WHITE,

            default_text: Text::new("", ""),
            user_text: Text::new("", text.as_str()),

            input_buffer: String::from(""),

            on_text_changed: None,
            on_text_submit: None,
        }
    }

    pub fn with_default_text(mut self, text: &str) -> Self {
        let mut owned_text = Text::new("", text)
            .with_color(colors::DARK_GRAY);

        // TODO: How to calculate this?
        // TODO: Should the size of this widget be based on text dimensions?
        owned_text.place(self.rect.x() + 5, self.rect.y);

        self.default_text = owned_text;
        self
    }

    /// Interact with mutable state reference when text input changes
    pub fn with_on_text_changed<F: 'static + Fn(&mut T, String)>
    (mut self, callback: F) -> Self 
    {
        self.on_text_changed = Some(Box::new(callback));
        self
    }

    /// Called when Enter key is pressed on focused TextBox
    /// - "Submits" TextBox content
    pub fn with_on_text_submit<F: 'static + Fn(&mut T, String)>
    (mut self, callback: F) -> Self
    {
        self.on_text_submit = Some(Box::new(callback));
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
        self.focus_color
    }

    fn hover_color(&self) -> Color {
        self.background_color
    }

    fn text_component(&mut self) -> Option<&mut Text<T>> {
        // FIXME: Clean this up

        // Sizes user text if any, otherwise sizes default text if any
        if self.user_text.text != "" {
            return Some(&mut self.user_text);
        }
        else if self.default_text.text != "" {
            return Some(&mut self.default_text);
        }
        None
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        // Sizes user text if any, otherwise sizes default text if any
        // Never need both to be sized at once, so this works
        
        // FIXME: This should not expect. Use if-let instead.
        self.text_component().expect("Failed to obtain text").assign_text_dimensions(dims);
    }

    fn render(&self, window: &mut Window<T>, widget_state: WidgetState) 
    where T: super::GenerateView<T, T> {
        match widget_state {
            WidgetState::Focused => window.canvas.set_draw_color(self.focus_color),
            _ => window.canvas.set_draw_color(self.background_color),
        }

        // Draw the background
        window.canvas.fill_rect(self.rect).unwrap();
        
        // Draw user_text > default_text
        if self.user_text.text != "" {
            self.user_text.render(window, widget_state);
        }
        else if widget_state != WidgetState::Focused && self.default_text.text != "" {
            self.default_text.render(window, widget_state);
        }
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.rect = Rect::new(
            self.rect.x + dx,
            self.rect.y + dy,
            self.rect.width(),
            self.rect.height(),
        );

        self.default_text.translate(dx, dy);
        self.user_text.translate(dx, dy);
    }

    fn place(&mut self, x: i32, y: i32) {
        self.rect = Rect::new(x, y, self.rect.width(), self.rect.height());

        self.default_text.container_rect = self.rect;
        self.user_text.container_rect = self.rect;
    }

    fn draw_width(&self) -> u32 {
        self.rect.width()
    }

    fn draw_height(&self) -> u32 {
        self.rect.height()
    }

    fn on_click(&mut self, state: &mut T) {

    }

    fn can_focus(&self) -> bool {
        true
    }

    fn update(&mut self, state: &mut T, event: &Event) {
        // TODO: Do something else to avoid mutable state
        // TODO: Avoid using .clone()

        // Clone is only used because render() will not account for text-size changes after update
        match event {
            Event::TextInput { text, .. } => {
                if let Some(on_text_changed) = &self.on_text_changed {
                    self.input_buffer += text;
                    (on_text_changed)(state, self.user_text.text.clone() + &self.input_buffer);
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                let mut text = self.user_text.text.clone();
                text.pop();
                if let Some(on_text_changed) = &self.on_text_changed {
                    (on_text_changed)(state, text);
                }
            }
            Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                if let Some(on_text_submit) = &self.on_text_submit {
                    (on_text_submit)(state, self.user_text.text.clone())
                }
            }
            _ => {}
        } 
    } //end update
} // end impl Widget

impl<T> ViewComponent<T> for TextBox<T> where T: 'static {
    fn as_component(self) -> WidgetOrView<T> {
        WidgetOrView::Widget(Box::new(self))
    }
}