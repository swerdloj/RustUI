pub mod views;
pub mod widgets;

// ========================== ViewComponent enum ========================== //

/// Contains either a Widget or a View. Handle via `match`.
pub enum ViewComponent<T> {
    Widget(Box<dyn widgets::widget::Widget<T>>),
    View(Box<dyn views::view::View<T>>),
    Component(/* TODO: This */),
}

// ========================== IntoViewComponent trait ========================== //

/// Trait utilized for storing `Widget` and `View` types together
pub trait IntoViewComponent<T> {
    fn as_component(self) -> ViewComponent<T>;
}

// ========================== Padding Struct ========================== //

pub struct Padding {
    left: u32,
    right: u32,
    top: u32,
    bottom: u32,
}

// ========================== Default Colors ========================== //

/// Default color implementations
pub mod colors {
    extern crate sdl2;
    use sdl2::pixels::Color;

    /// `Color::RGB(0, 0, 0)` - Black
    pub const BLACK: Color = Color {r: 0, g: 0, b: 0, a: 0xff};
    /// `Color::RGB(50, 50, 100)` - Default background color
    pub const DARK_PURPLE: Color = Color {r: 50, g: 50, b: 100, a: 0xff};
    /// `Color::RGB(240, 240, 200)` - Default button color
    pub const MANILLA: Color = Color {r: 240, g: 240, b: 200, a: 0xff};
    /// `Color::RGB(255, 255, 255)` - White
    pub const WHITE: Color = Color {r: 255, g: 255, b: 255, a: 0xff};
    /// `Color::RGB(200, 200, 200)` - Light Gray
    pub const LIGHT_GRAY: Color = Color {r: 200, g: 200, b: 200, a: 0xff};
    /// `Color::RGB(80, 80, 80)` - Light Gray
    pub const DARK_GRAY: Color = Color {r: 80, g: 80, b: 80, a: 0xff};
}