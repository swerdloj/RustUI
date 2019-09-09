use std::collections::HashMap;

use crate::view::{Alignment, WidgetOrView};
use crate::widgets::widget::Widget;

/// Base trait from which `View` types are derived
pub trait View {
    /// Initializes the view, combining all subviews
    fn init(&mut self);

    /// Translates an entire view by dx & dy
    fn translate(&mut self, dx: i32, dy: i32);
    /// Aligns a view's components
    fn align(&mut self, alignment: Alignment);
    
    /// The width of the view (as drawn)
    fn draw_width(&self) -> u32;
    /// The height of the view (as drawn)
    fn draw_height(&self) -> u32;


    // --------- Builder Functions --------- //

    fn fixed_width(self, width: u32) -> Self;
    fn fixed_height(self, height: u32) -> Self;
    fn fixed_size(self, width: u32, height: u32) -> Self;
}

// FIXME: This is mirrored in /widgets/widget.rs
pub struct Padding {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

pub struct ViewData<T> {
    // id: &'static str,

    /// Map of user-assigned widget names -> widget
    pub component_map: HashMap<u32, Box<dyn Widget<T>>>,
    /// The items (widgets or nested views) owned by the view
    pub components: Vec<WidgetOrView<T>>,

    // TODO: Consider replacing these with functions
    //       Although some way to contain manual sizes are needed
    /// Assigned width of view 
    pub view_width: u32,
    /// Assigned height of view
    pub view_height: u32,
    /// Whether the view has manually-fixed sizes
    pub fixed_size: bool,

    /// The view's padding (not the widgets')
    pub padding: Padding,
    /// View alignment
    pub alignment: Alignment,
}