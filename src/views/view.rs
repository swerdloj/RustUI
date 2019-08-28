use super::view::Alignment;

/// Base trait from which `View` types are derived
pub trait View {
    /// Translates an entire view by dx & dy
    fn translate(&mut self, dx: i32, dy: i32);
    /// Aligns a view's components
    fn align(&mut self, alignment: Alignment);
    
    // TODO: Are these needed here?
    fn draw_width();
    fn draw_height();
}