extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::event::Event;

use crate::{colors, Window};

use super::{Widget, WidgetState};
use crate::view_components::{IntoViewComponent, ViewComponent};
use crate::images;

use std::path::Path;


/// Image Widget
/// - Displays an image
pub struct Image<T> {
    id: &'static str,
    rect: Rect,
    // resource_path: String,
    image_surface: Surface<'static>,

    // Let user decide whether hovering image highlight border it or adjusts image (e.g.: darken)
    // TODO: Add these options to a mod.rs style enum: 'Border', 'Darken' or something like that
    hover_border: bool,
    hover_border_width: u32,
    hover_color: Color,
    click_color: Color,
    
    // Interact with state when image is clicked
    on_click: Option<Box<dyn Fn(&mut T)>>,
}

impl<T> Image<T> {
    /// - `resource_path`: Path to image resource as static string
    /// - `bounds`: (width, height) bounds for image
    pub fn new(id: &'static str, resource_path: &'static str, bounds: (u32, u32)) -> Self {
        // FIXME: I am doing this here to obtain image dimensions before view init. This is not final.
        let surface = images::load_image(&Path::new(resource_path)).expect("Failed to load resource");

        // TODO: These need to be scaled to image's aspect ration (query dimensions first)
        let (width, height) = bounds;

        Image {
            id: id,
            rect: Rect::new(0, 0, width, height),
            // resource_path: resource_path,
            image_surface: surface,
            hover_border: true,
            hover_border_width: 6,
            hover_color: colors::DARKER_PURPLE,
            click_color: colors::BLACK,
            on_click: None,
        }
    }

    /// Assign on_click function
    pub fn with_on_click<F: Fn(&mut T) + 'static>(mut self, callback: F) -> Self {
        self.on_click = Some(Box::new(callback));
        self
    }

    /// Draw a border around image when hovered
    pub fn with_hover_border(mut self) -> Self {
        self.hover_border = true;
        self
    }

    /// Width of border around image on hover if enabled
    pub fn with_hover_border_width(mut self, width: u32) -> Self {
        self.hover_border_width = width;
        self
    }

    /// Shade the image when hovered
    // TODO: Consider having user pass colors here
    pub fn with_hover_shade(mut self) -> Self {
        self.hover_border = false;
        self.hover_color = Color::RGBA(30, 30, 80, 100);
        self.click_color = Color::RGBA(20, 20, 50, 160);
        self
    }

    /// On-hover border or shade color (with alpha channel)
    pub fn with_hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }
    
    /// On-click border or shade color (with alpha channel)
    pub fn with_click_color(mut self, color: Color) -> Self {
        self.click_color = color;
        self
    }
}

impl<T> Widget<T> for Image<T> {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn id(&self) -> &'static str {
        self.id
    }

    fn text_component(&mut self) -> Option<&mut super::Text<T>> {
        None
    }

    fn place(&mut self, x: i32, y: i32) {
        self.rect.set_x(x);
        self.rect.set_y(y);
    }

    fn on_click(&mut self, state: &mut T) {
        if let Some(on_click) = &self.on_click {
            (on_click)(state);
        }
    }

    // fn update(&mut self, state: &mut T, event: &Event) {
        
    // }

    fn render(&self, window: &mut Window<T>, widget_state: WidgetState)
    where T: super::GenerateView<T> {
        // FIXME: There is a lot here that can be refactored and made more efficient

        let mut draw_highlight = false;
        let mut clicking = false;

        match widget_state {
            WidgetState::Hovering => {
                window.canvas.set_draw_color(self.hover_color);

                if !self.hover_border {
                    draw_highlight = true;
                }
            }
            WidgetState::Active => {
                window.canvas.set_draw_color(self.click_color);

                if !self.hover_border {
                    draw_highlight = true;
                    clicking = true;
                }
            }
            _ => {}
        }

        if !draw_highlight { // border
            let border = Rect::new(self.rect.x() - self.hover_border_width as i32,  self.rect.y() - self.hover_border_width as i32,
                                   self.rect.width() + self.hover_border_width * 2, self.rect.height() + self.hover_border_width * 2);
            window.canvas.fill_rect(border).unwrap();
        }

        // FIXME: Clean up rendering (no memory leaks apparent)

        let texture_creator = window.canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&self.image_surface)
                        .expect("Failed to create image texture");

        // let sdl2::render::TextureQuery { width, height, ..} = texture.query();

        // FIXME: These will be scaled according to aspect ration later (done in Self::new())
        let target = Rect::new(self.rect.x(), self.rect.y(), self.rect.width(), self.rect.height());

        window.canvas.copy(&texture, None, Some(target)).expect("Failed to copy texture to target");
    
        if draw_highlight {
            // FIXME: Instead, render colored RGBA rect as surface, then blit with image.
            window.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
            window.canvas.fill_rect(self.rect).unwrap();
            window.canvas.set_blend_mode(sdl2::render::BlendMode::None);
        }
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.rect.set_x(self.rect.x + dx);
        self.rect.set_y(self.rect.y + dy);
    }

    fn draw_width(&self) -> u32 {
        self.rect.width()
    }
    
    fn draw_height(&self) -> u32 {
        self.rect.height()
    }
}

impl<T> IntoViewComponent<T> for Image<T> where T: 'static {
    fn as_component(self) -> ViewComponent<T> {
        ViewComponent::Widget(Box::new(self))
    }
}