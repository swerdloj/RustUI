/*

Widget functionality

Jonathan Swerdlow
7-2-19
*/

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

/// This is the base widget struct from which all other widgets are derived
/// # Arguments
/// 
/// * `x` - The widget's top-left x location within the window
/// * `y` - The widget's top-left y location within the window
/// * `id` - The widget's id as a string TODO: This should be a hash of the string instead for faster lookup.
pub struct WidgetData {
    rect: Rect,
    // TODO: Colors?
    pub id: &'static str,
}

// TODO: See view.rs

pub struct Button {
    pub id: u32,
    pub rect: Rect,
    pub is_active: bool,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub hover_color: Color,
    pub on_click: &'static Fn(), // TODO: Move this over to the trait below and allow the user to implement this??
}

impl Widget for Button {
    // TODO: This
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn primary_color(&self) -> Color {
        self.primary_color
    }

    fn secondary_color(&self) -> Color {
        self.secondary_color
    }

    fn hover_color(&self) -> Color {
        self.hover_color
    }

    fn on_click(&self) {
        (self.on_click)();
    }
}

pub struct Text {
    pub id: u32,
    pub rect: Rect,
    // TODO: Implement text (sdl2 ttf extension)
}

impl Widget for Text {
    // TODO: This

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn primary_color(&self) -> Color {
        Color::RGB(0, 0, 0)
    }

    fn secondary_color(&self) -> Color {
        Color::RGB(0, 0, 0)
    }

    fn hover_color(&self) -> Color {
        Color::RGB(0, 0, 0)
    }

}

pub trait Widget {
    fn get_rect(&self) -> Rect;
    fn get_id(&self) -> u32;
    fn primary_color(&self) -> Color;
    fn secondary_color(&self) -> Color;
    fn hover_color(&self) -> Color; // TODO: This

    fn on_click(&self) {}

    /// Instatiate the widget at the given (x, y) coordinate with an optional id
    fn new(x: i32, y: i32, id: &str) 
    where Self: Sized
    {
        
    }

    /// Draw the widget to the window
    fn draw() 
    where Self: Sized
    {

    }

    // TODO: Inputs & return types (pass mouse locations, keys pressed, etc.)


    fn on_hover() 
    where Self: Sized
    {
        // Mouse hovers over widget
    }

    // fn on_click() 
    // where Self: Sized
    // {
    //     // User clicks, then releases the mouse
    // }

    fn on_mouse_down() 
    where Self: Sized
    {
        // User clicks (consider this for widgets such as sliders)
    }

    fn on_mouse_up() 
    where Self: Sized
    {
        // User releasese mouse (consider this for widgets such as sliders)
    }

    fn on_key_down() 
    where Self: Sized
    {
        // The user presses down a key with the current widget selected (TODO: How to select a widget? 'Highlight current'?)
    }

    fn on_key_up() 
    where Self: Sized
    {
        // The user releases a pressed key (see on_key_down)
    }
}