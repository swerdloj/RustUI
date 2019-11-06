extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::{Window, colors};
use super::Component;
use super::super::{Orientation, IntoViewComponent, ViewComponent};


pub struct Divider<T> {
    x: i32,
    y: i32,

    orientation: Orientation,
    thickness: u32,
    color: Color,

    padding_before: u16,
    padding_after: u16,

    state_type: std::marker::PhantomData<T>,
}

impl<T> Divider<T> {
    pub fn new(orientation: Orientation) -> Self {
        Divider {
            x: 0,
            y: 0,

            orientation: orientation,
            thickness: 4,
            color: colors::DARKER_PURPLE,
            
            padding_before: 20,
            padding_after: 20,

            state_type: std::marker::PhantomData,
        }
    }

    pub fn with_thickness(mut self, thickness: u32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl<T> Component<T> for Divider<T> {
    fn render(&self, window: &mut Window<T>, parent_dimensions: (u32, u32))
    where T: crate::state::GenerateView<T> {
        
        window.canvas.set_draw_color(self.color);
        
        match self.orientation {
            Orientation::Horizontal => {
                let y = self.y + (self.draw_height() as i32 / 2);
                window.canvas.fill_rect(Rect::new(self.x, y, parent_dimensions.0, self.thickness)).unwrap();
            }
            Orientation::Vertical => {
                let x = self.x + (self.draw_width() as i32 / 2);
                window.canvas.fill_rect(Rect::new(x, self.y, self.thickness, parent_dimensions.1)).unwrap();
            }
        }
    }

    fn place(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    fn draw_width(&self) -> u32 {
        match self.orientation {
            Orientation::Horizontal => {
                0
            }
            Orientation::Vertical => {
                (self.padding_before + self.padding_after) as u32 + self.thickness
            }
        }
    }
    
    fn draw_height(&self) -> u32 {
        match self.orientation {
            Orientation::Horizontal => {
                (self.padding_before + self.padding_after) as u32 + self.thickness
            }
            Orientation::Vertical => {
                0
            }
        }
    }
}

impl<T> IntoViewComponent<T> for Divider<T> where T: 'static {
    fn as_component(self) -> ViewComponent<T> {
        ViewComponent::Component(Box::new(self))
    }
}