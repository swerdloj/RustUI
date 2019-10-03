extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::view_components::{ViewComponent, WidgetOrView};
use crate::font::FontParams;
use crate::backend::system::window::Window;

use super::widget::{Widget, WidgetState, colors};

// See https://github.com/Rust-SDL2/rust-sdl2/blob/master/src/sdl2/ttf/font.rs for help
// TTF is undocumented on sdl2 crate docs.


pub struct Text<T> {
    id: &'static str,
    pub container_rect: Rect,
    primary_color: Color,
    pub text: String,
    pub font: FontParams,
    // How far text must be from its boundary
    internal_padding: u32,

    update_fn: Option<Box<dyn Fn(&T) -> String>>,

    auto_resize: bool,
    center_text: bool,
   
    // Text surface parameters    
    text_width: u32,
    text_height: u32,
}

impl<T> Text<T> {
    pub fn new(id: &'static str, text: &str) -> Self {
        Text {
            id: id,
            container_rect: Rect::new(0, 0, 0, 28), // FIXME: 28 is only true for default font
            primary_color: colors::BLACK,
            text: String::from(text),
            font: FontParams::default_font(),
            internal_padding: 10,
            update_fn: None,
            auto_resize: false,
            center_text: false,
            // FIXME: Defaults are not safe. Should be assigned when building view
            text_width: 100,
            text_height: 40,
        }
    }

    pub fn center(mut self) -> Self {
        self.center_text = true;
        self
    }

    pub fn with_text_update<F: 'static + Fn(&T) -> String>
    (mut self, update_fn: F) -> Self  
    {
        self.update_fn = Some(Box::new(update_fn));
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.primary_color = color;
        self
    }

    pub fn with_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.primary_color = Color::RGB(r, g, b);
        self
    }

    pub fn with_rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.primary_color = Color::RGBA(r, g, b, a);
        self
    }

    pub fn auto_resize(mut self) -> Self {
        self.auto_resize = true;
        self
    }

    // TODO: This only rescales the text in one dimension. It should rescale both dimensions by the same factor
    // TODO: Update this with padding_left, padding_right, padding_top, padding_bottom when implemented
    fn fit_and_center_within_container(&self, container_rect: &Rect) -> Rect {
        let width_constraint = (container_rect.width() as i32 - self.internal_padding as i32) as u32;
        let height_constraint = (container_rect.height() as i32 - self.internal_padding as i32) as u32;
        
        // Determine whether the text is too tall and/or too wide
        let width_ratio = self.text_width as f32 / width_constraint as f32;
        let height_ratio = self.text_height as f32 / height_constraint as f32;

        // Downscale the text according to the largest out-of-bounds dimension
        // see https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/ttf-demo.rs
        let (width, height) = if width_ratio > 1f32 || height_ratio > 1f32 {
            if width_ratio > height_ratio {
                // let new_width = 0;
                let new_height = (height_constraint as f32 / width_ratio) as u32;
                (width_constraint, new_height)
            } else { // height_ratio is larger 
                let new_width = (width_constraint as f32 / height_ratio) as u32;
                (new_width, height_constraint)
            }
        } else { // The text is already a good size
            (self.text_width, self.text_height)
        };
        
        // Center the text within its boundary (such as a button's rect)
        // See answer https://stackoverflow.com/questions/27912979/center-rectangle-in-another-rectangle
        let result = Rect::new(
            container_rect.x + ((container_rect.width() as i32 - width as i32) / 2),
            container_rect.y + ((container_rect.height() as i32 - height as i32) / 2),
            width,
            height
        );

        return result;
    }
}

impl<T> Widget<T> for Text<T> {
    fn assign_id(&mut self, id: &'static str) {
        self.id = id;
    }

    fn place(&mut self, x: i32, y: i32) {
        self.container_rect = Rect::new(x, y, self.container_rect.width(), self.container_rect.height());
    }

    fn text_component(&mut self) -> Option<&mut Text<T>> {
        Some(self)
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        self.text_width = dims.0;
        self.text_height = dims.1;
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.container_rect = Rect::new(
            self.rect().x() + dx,
            self.rect().y() + dy,
            self.rect().width(),
            self.rect().height()
        );
    }

    fn rect(&self) -> Rect {
        self.container_rect
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn primary_color(&self) -> Color {
        self.primary_color
    }

    fn secondary_color(&self) -> Color {
        Color::RGB(0, 0, 0)
    }

    fn hover_color(&self) -> Color {
        Color::RGB(0, 0, 0)
    }

    fn on_click(&mut self, state: &mut T) {

    }

    // TODO: Resize the text surface on update
    fn update(&mut self, state: &T) {
        if let Some(ref update_callback) = self.update_fn {
            self.text = (update_callback)(state);
        }
    }

    fn render(&self, window: &mut Window<T>, widget_state: WidgetState)
    where T: super::GenerateView<T, T> {
        // FIXME: Allocating texture_creator here is probably bad if we use it each render
        let texture_creator = window.canvas.texture_creator();

        // FIXME: Same here.
        // TODO: Implement font.rs
        let font = window.ttf_context.load_font(
            std::path::Path::new("./res/font/OpenSans-Regular.ttf"), 
            20
        ).expect("Failed to load font");

        let surface = font.render(&self.text)
            .blended(self.primary_color).unwrap();

        let texture = texture_creator.create_texture_from_surface(surface).expect("Failed to create texture");
        // let TextureQuery { width, height, .. } = texture.query();

        // println!("Rendering '{}' with containter {}x{} at ({}, {}) and text size {}x{}", self.text, self.container_rect.width(), self.container_rect.height(), self.container_rect.x(), self.container_rect.y(), self.text_width, self.text_height);

        let target = if self.auto_resize {
            // Center text within container & downscale if too large
            self.fit_and_center_within_container(&self.container_rect)
        } else if self.center_text {
            // Center the text, disregarding containers
            let center_x = self.container_rect.x() + self.container_rect.width() as i32 / 2;
            let target_x = center_x - self.text_width as i32 / 2;
            Rect::new(
                target_x,
                self.container_rect.y() + ((self.container_rect.height() as i32 - self.text_height as i32) / 2),
                self.text_width,
                self.text_height
            )
        } else {
            // Center the text's y position and align left
            Rect::new(
                self.container_rect.x(),
                self.container_rect.y() + ((self.container_rect.height() as i32 - self.text_height as i32) / 2),
                self.text_width,
                self.text_height
            )
        };

        window.canvas.copy(&texture, None, Some(target)).unwrap();
    }

    fn draw_width(&self) -> u32 {
        self.text_width
    }

    fn draw_height(&self) -> u32 {
        self.text_height
    }
}

impl<T> ViewComponent<T> for Text<T> where T: 'static {
    fn as_component(self) -> WidgetOrView<T> {
        WidgetOrView::Widget(Box::new(self))
    }
}