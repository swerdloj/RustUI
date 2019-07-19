/*

Widget functionality

Jonathan Swerdlow
7-2-19

*/

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;
// use sdl2::render::WindowCanvas as Canvas;
use crate::backend::system::window::Window;


pub enum WidgetState {
    // TODO: Should the state be called 'Active' or 'Clicking'?
    //       or should these be two different states?
    Active,
    Hovering,
    Base,
    // can include structs, algebraic types, etc.
}

/// This is the base widget struct from which all other widgets are derived
/// # Arguments
/// 
/// * `id` - The widget's id as a string TODO: This should be a hash of the string instead for faster lookup.
pub struct WidgetData {
    // TODO: Ensure the id is truly unique
    id: u32, // The widget's *unique* id
    rect: Rect, // Width, height, and location
    primary_color: Color, // The widget's base color (e.g.: button base color or text color)
}

// TODO: See view.rs

// TODO: When instiating a widget, this is potential syntax via a builder:
// Button::new().primary_color(...).padding(...).hover_color(...)
// Consider replacing Button::new() with just 'Button!' to keep the syntax design philosophy in tact
// Doing the above will allow for me to implement default values without worrying about complexity/viability

// TODO: Consider accepting the sdl2 canvas into a render function as well as relevant state information
// This will avoid requiring things like "secondary_color" for widgets like text where this doesn't make sense
// This will also allow for custom logic when dealing with unique widgets (rather than treating them all the same)

// TODO: Replace 'T' with 'S' for the sake of clarity?
// NOTE: In this module, the generic type 'T' refers EXCLUSIVELY to user-defined state

pub struct Button<T> {
    pub id: u32,
    pub rect: Rect,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub hover_color: Color,
    pub text: Option<Text>,
    pub on_click: Option<Box<Fn(&mut T)>>,
}

// FIXME: .with_rect() & .with_id() are only for testing purposes. The user should never access these
impl<T> Button<T> {
    // TODO: How to adjust these?? Keeping them default like this can't be good unless the view adjusts it
    pub fn new(id: &str) -> Self {
        Button {
            id: 0,
            rect: Rect::new(0, 0, 0, 0),
            primary_color: Color::RGB(240, 240, 200),
            secondary_color: Color::RGB(100, 100, 100),
            hover_color: Color::RGB(200, 200, 200),
            text: None,
            on_click: None,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        // TODO: How to hanle the sub-widget's id?
        //       Note that the sub-widget is not actually part of the view
        self.text = Some(Text::new("", text));
        self
    }

    pub fn with_on_click(mut self, callback: Box<Fn(&mut T)>) -> Self
    {
        self.on_click = Some(callback);
        self
    }

    // TODO: The user should not need this (delete this later)
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    // TODO: id should be hashed from new(str), delete this later
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
}

impl<T> Widget<T> for Button<T> {
    fn render(&self, window: &mut Window<T>, widget_state: WidgetState) {
        match widget_state {
            WidgetState::Hovering => window.canvas.set_draw_color(self.hover_color),
            WidgetState::Active => window.canvas.set_draw_color(self.secondary_color),
            _ => window.canvas.set_draw_color(self.primary_color),
        }

        window.canvas.fill_rect(self.rect).unwrap();
        // pay attention to draw order
        if let Some(button_text) = &self.text {
            button_text.render(window, widget_state);
        }
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn id(&self) -> u32 {
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

    fn on_click(&self, state: &mut T) {
        if let Some(ref on_click_function) = self.on_click {
            (on_click_function)(state);
        }
    }
}

pub struct Text {
    id: u32,
    rect: Rect,
    primary_color: Color,
    // font: 
    text: String,
}

impl Text {
    pub fn new(id: &str, text: &str) -> Self {
        Text {
            id: 100,
            rect: Rect::new(0, 0, 0, 0),
            primary_color: Color::RGB(0, 0, 0),
            text: String::from(text),
        }
    }

    // TODO: The user should not need this (delete this later)
    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    // TODO: id should be hashed from new(str), delete this later
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
}

// TODO: The Widget trait is only for characteristics shared by ALL widgets
impl<T> Widget<T> for Text {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn primary_color(&self) -> Color {
        self.primary_color
    }

    fn secondary_color(&self) -> Color {
        Color::RGB(0, 0, 0)
    }

    fn hover_color(&self) -> Color {
        Color::RGB(0, 0, 0)
    }

    fn on_click(&self, state: &mut T) {

    }

    fn render(&self, window: &mut Window<T>, widget_state: WidgetState) {
        // FIXME: Allocating texture_creator here is probably bad if we use it each render
        let texture_creator = window.canvas.texture_creator();
        // FIXME: Same here. Consider storing loaded fonts into some data structure
        let font = window.ttf_context.load_font(
            std::path::Path::new("./res/font/OpenSans-Regular.ttf"), 
            20
        ).expect("Failed to load font. Ensure the font path was correct.");

        let surface = font.render(&self.text)
            .blended(self.primary_color).unwrap();

        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(30, 30, width, height);

        window.canvas.copy(&texture, None, Some(target)).unwrap();
    }
}

pub trait Widget<T> {
    fn rect(&self) -> Rect;
    fn id(&self) -> u32;
    fn primary_color(&self) -> Color;
    fn secondary_color(&self) -> Color;
    fn hover_color(&self) -> Color; // TODO: This

    fn on_click(&self, state: &mut T);

    // fn render(&self, canvas: &mut Canvas, widget_state: WidgetState);
    fn render(&self, window: &mut Window<T>, widget_state: WidgetState);

    /// Instatiate the widget with the given id.
    /// All widget fields are filled with defaults. Builder methods may be used to adjust these fields.
    
    // TODO: Can I not require Self::new as part of the trait?
    // fn new(id: &str) -> Self
    // where Self: Sized;

    /// Render the widget to the window


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