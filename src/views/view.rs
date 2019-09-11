/*

View functionality

All views must implement the View trait.

When applicable, include ViewData & Padding structs.

*/

use std::collections::HashMap;

use crate::widgets::widget::Widget;

// ========================== WidgetOrView enum ========================== //

/// Contains either a Widget or a View. Handle via `match`.
pub enum WidgetOrView<T> {
    Widget(Box<dyn Widget<T>>),
    View(Box<dyn View>),
}

// ========================== ViewComponent trait ========================== //

/// Trait utilized for storing `Widget` and `View` types together
pub trait ViewComponent<T> {
    fn as_component(self) -> WidgetOrView<T>;
}

// ========================== Alignment enum ========================== //

/// View alignments
/// ## Alignments
/// * `Left` - Align each widget to the left within its view (default)
/// * `Center` - Center each widget within its view
/// * `Right` - TODO: This
pub enum Alignment {
    Center,
    Left,
    Right,
}

// ========================== View Trait ========================== //

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

    // fn widgets_mut(&mut self) -> Vec<&mut dyn Widget<T>>;


    // --------- Builder Functions --------- //

    // TODO: Only the **window** should have these fields
    //       Refactor these to the backend `Window`
    fn fixed_width(self, width: u32) -> Self where Self: Sized;
    fn fixed_height(self, height: u32) -> Self where Self: Sized;
    fn fixed_size(self, width: u32, height: u32) -> Self where Self: Sized;
}

// FIXME: This is mirrored in /widgets/widget.rs
pub struct Padding {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

/// Common data needed by *all* View structs
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