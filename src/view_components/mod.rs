pub mod views;
pub mod widgets;

use views::view::View;
use widgets::widget::Widget;

// ========================== WidgetOrView enum ========================== //


/// Contains either a Widget or a View. Handle via `match`.
pub enum WidgetOrView<T> {
    Widget(Box<dyn Widget<T>>),
    View(Box<dyn View<T>>),
}

// ========================== ViewComponent trait ========================== //

/// Trait utilized for storing `Widget` and `View` types together
pub trait ViewComponent<T> {
    fn as_component(self) -> WidgetOrView<T>;
}

// ========================== Padding Struct ========================== //

pub struct Padding {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}