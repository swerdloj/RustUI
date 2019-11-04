/*

View functionality

All views must implement the View trait.

When applicable, include ViewData & Padding structs.

*/
extern crate sdl2;
use sdl2::ttf;

use std::collections::HashMap;

use crate::view_components::ViewComponent;
use crate::view_components::widgets::widget::Widget;
use crate::view_components::components::Component;

// ========================== Alignment enum ========================== //

/// View alignments
/// ## Alignments
/// * `Left` - Align each widget to the left within its view (default)
/// * `Center` - Center each widget within its view
/// * `Right` - TODO: This
#[derive(Clone, Copy)]
pub enum Alignment {
    Center,
    Left,
    Right,
}

// ========================== View Trait ========================== //

/// Base trait from which `View` types are derived
pub trait View<T> {
    /// Initializes the view, combining all subviews
    fn init(&mut self, ttf_context: &ttf::Sdl2TtfContext);

    /// Translates an entire view by dx & dy
    fn translate(&mut self, dx: i32, dy: i32);
    /// Aligns a view's components
    fn align(&mut self);
    
    /// The width of the view (as drawn)
    fn draw_width(&self) -> u32;
    /// The height of the view (as drawn)
    fn draw_height(&self) -> u32;

    /// The actual size of the view. 
    /// Accounts for fixed dimensions unlike `draw_width()` & `draw_height()`
    fn view_size(&self) -> (u32, u32);

    /// Obtain mutable references to all of a view's widgets
    fn widgets_mut(&mut self) -> Vec<&mut Box<dyn Widget<T>>>;
    /// Obtain references to all of a view's widgets
    fn widgets(&self) -> Vec<&Box<dyn Widget<T>>>;

    /// Obtain mutable references to *all* nested widgets
    fn child_widgets_mut(&mut self) -> Vec<&mut Box<dyn Widget<T>>>;
    
    // TODO: rename/fix this
    fn child_comps(&self) -> Vec<&Box<dyn Component<T>>>;
    

    // --------- Builder Functions --------- //

    // TODO: Only the **window** should have these fields
    //       Refactor these to the backend `Window`
    fn alignment(self, alignment: Alignment) -> Self where Self: Sized;
    fn fixed_width(self, width: u32) -> Self where Self: Sized;
    fn fixed_height(self, height: u32) -> Self where Self: Sized;
    fn fixed_size(self, width: u32, height: u32) -> Self where Self: Sized;
    /// (left, right, top, bottom)
    fn padding(self, left: u32, right: u32, top: u32, botton: u32) -> Self where Self: Sized;
}

// TODO: Many copy/pasted functions can be applied to this struct
//  rather than each individual view (anything that uses only view.data)
/// Common data needed by all View structs
pub struct ViewData<T> {
    // id: &'static str,

    /// Map of user-assigned widget ids -> widgets
    pub component_map: HashMap<&'static str, Box<dyn Widget<T>>>,
    /// The items (widgets or nested views) owned by the view
    pub components: Vec<ViewComponent<T>>,

    /// View's draw width unless manually assigned
    pub view_width: u32,
    /// View's draw height unless manually assigned
    pub view_height: u32,
    /// Whether the view has manually-fixed sizes (view_width or view_height)
    // FIXME: This is never used
    pub fixed_size: bool,

    /// View alignment
    pub alignment: Alignment,
}