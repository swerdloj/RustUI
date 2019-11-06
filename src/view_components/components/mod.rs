pub mod divider;

use crate::Window;


pub use divider::Divider;

/// Base trait from which view components are derived
pub trait Component<T> {
    /// Place component by its upper-left point
    fn place(&mut self, x: i32, y: i32);
    /// Render the component to the window
    fn render(&self, window: &mut Window<T>, parent_dimensions: (u32, u32))
    where T: crate::state::GenerateView<T>;

    /// Drawn width of component
    fn draw_width(&self) -> u32;
    /// Drawn height of component
    fn draw_height(&self) -> u32;
}