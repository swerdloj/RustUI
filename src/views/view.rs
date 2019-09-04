use std::collections::HashMap;

use super::view::{Alignment, WidgetOrView};
use super::widgets::widget::Widget;

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

// FIXME: This is mirrored in /widgets/widget.rs
pub struct Padding {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

pub struct ViewData {
    /// Map of user-assigned widget names -> widget
    component_map: HashMap<u32, Box<dyn Widget<T>>>,
    /// The items (widgets or nested views) owned by the view
    components: Vec<WidetOrView<T>>,

    // TODO: Consider replacing these with functions
    //       Although some way to contain manual sizes are needed
    /// Assigned width of view 
    pub view_width: u32,
    /// Assigned height of view
    pub view_height: u32,
    /// Whether the view has manually-fixed sizes
    fixed_size: bool,

    /// The view's padding (not the widgets')
    view_padding: Padding,
    /// View alignment
    alignment: Alignment,

}