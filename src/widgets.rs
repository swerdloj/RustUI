/*

Widget functionality

Jonathan Swerdlow
7-2-19

*/

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;
use crate::backend::system::window::Window;

/// Possible widget states
/// ## States
/// * `Active` - The widget is currently being clicked
/// * `Hovering` - The widget is being hovered over
/// * `Base` - The widget is in its default state
pub enum WidgetState {
    // TODO: Should the state be called 'Active' or 'Clicking'?
    //       or should these be two different states?
    Active,
    Hovering,
    Base,
    // can include structs, algebraic types, etc.
}

/// This is the base widget struct from which all other widgets are derived
/// ## Arguments
/// * `id` - The widget's id as a string TODO: This should be a hash of the string instead for faster lookup.
struct WidgetData {
    // TODO: Ensure the id is truly unique
    id: u32, // The widget's *unique* id
    rect: Rect, // Width, height, and location
    primary_color: Color, // The widget's base color (e.g.: button base color or text color)
}

// TODO: Consider callback types: https://oribenshir.github.io/afternoon_rusting/blog/closures

// TODO: When instiating a widget, this is potential syntax via a builder:
// Button::new().primary_color(...).padding(...).hover_color(...)
// Consider replacing Button::new() with just 'Button!' to keep the syntax design philosophy in tact
// Doing the above will allow for me to implement default values without worrying about complexity/viability

// TODO: Consider accepting the sdl2 canvas into a render function as well as relevant state information
// This will avoid requiring things like "secondary_color" for widgets like text where this doesn't make sense
// This will also allow for custom logic when dealing with unique widgets (rather than treating them all the same)

// TODO: Replace 'T' with 'S' for the sake of clarity?
// NOTE: In this module, the generic type 'T' refers EXCLUSIVELY to user-defined state

// ========================== BUTTON WIDGET ========================== //

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
            rect: Rect::new(0, 0, 100, 40),
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

    // TODO: id should be hashed from new(str), delete this later
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn place(mut self, x: i32, y: i32) -> Self {
        // Place the button at (x, y)
        self.rect = Rect::new(x, y, self.rect.width(), self.rect.height());

        // Place the button text respective of this new position
        if let Some(button_text) = &mut self.text {
            *button_text.rect = *self.rect;
        }

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

// ========================== TEXT WIDGET ========================== //

// TODO: Implement a text callback which takes in the state and updates the text accordingly
//       Consider an "on_state_changed" callback
pub struct Text {
    id: u32,
    rect: Rect,
    primary_color: Color,
    text: String,
    // How far text must be from its boundary
    internal_padding: u32,
}

impl Text {
    pub fn new(id: &str, text: &str) -> Self {
        Text {
            id: 100,
            rect: Rect::new(0, 0, 100, 40),
            primary_color: Color::RGB(0, 0, 0),
            text: String::from(text),
            internal_padding: 10,
        }
    }

    // TODO: id should be hashed from new(str), delete this later
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn place(mut self, x: i32, y: i32) -> Self {
        self.rect = Rect::new(x, y, self.rect.width(), self.rect.height());
        self
    }

    // TODO: This only rescales the text in one dimension. It should rescale both dimensions by the same factor
    // TODO: Update this with padding_left, padding_right, padding_top, padding_bottom when implemented
    fn fit_and_center_within_container(&self, text_width: u32, text_height: u32, container_rect: &Rect) -> Rect {
        let width_constraint = (container_rect.width() as i32 - self.internal_padding as i32) as u32;
        let height_constraint = (container_rect.height() as i32 - self.internal_padding as i32) as u32;
        
        // Determine whether the text is too tall and/or too wide
        let width_ratio = text_width as f32 / width_constraint as f32;
        let height_ratio = text_height as f32 / height_constraint as f32;

        // Downscale the text according to the largest out-of-bounds dimension
        // see https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/ttf-demo.rs
        let (width, height) = if width_ratio > 1f32 || height_ratio > 1f32 {
            if width_ratio > height_ratio {
                // let new_width = 0;
                let new_height = (height_constraint as f32 / width_ratio) as u32;
                (width_constraint, new_height)
            } else { // height_ratio is larger 
                let new_width = (width_constraint as f32 / height_ratio) as u32;
                (new_width, height_constraint)
            }
        } else { // The text is already a good size
            (text_width, text_height)
        };
        
        // Center the text within its boundary (such as a button's rect)
        // See answer https://stackoverflow.com/questions/27912979/center-rectangle-in-another-rectangle
        let result = Rect::new(
            container_rect.x + ((container_rect.width() as i32 - width as i32) / 2),
            container_rect.y + ((container_rect.height() as i32 - height as i32) / 2),
            width,
            height
        );

        return result;
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
        // TODO: How to handle different text sizes?
        let font = window.ttf_context.load_font(
            std::path::Path::new("./res/font/OpenSans-Regular.ttf"), 
            20
        ).expect("Failed to load font");

        let surface = font.render(&self.text)
            .blended(self.primary_color).unwrap();

        let texture = texture_creator.create_texture_from_surface(&surface).expect("Failed to create texture");
        let TextureQuery { width, height, .. } = texture.query();

        let target = self.fit_and_center_within_container(width, height, &self.rect);

        //let target = Rect::new(self.rect.x, self.rect.y, width, height);

        window.canvas.copy(&texture, None, Some(target)).unwrap();
    }
}

// ========================== WIDGET TRAIT ========================== //

pub trait Widget<T> {
    fn rect(&self) -> Rect;
    fn id(&self) -> u32;
    fn primary_color(&self) -> Color;
    fn secondary_color(&self) -> Color;
    fn hover_color(&self) -> Color;

    // fn place(&mut self, x: i32, y: i32);

    fn on_click(&self, state: &mut T);

    /// Render the widget to the window
    fn render(&self, window: &mut Window<T>, widget_state: WidgetState);

    /// Instatiate the widget with the given id.
    /// All widget fields are filled with defaults. Builder methods may be used to adjust these fields.
    // TODO: Can I not require Self::new as part of the trait because of 'Self'?
    // fn new(id: &str) -> Self
    // where Self: Sized;

    // TODO: Inputs & return types (pass mouse locations, keys pressed, etc.)

    fn on_hover() 
    where Self: Sized
    {
        // Mouse hovers over widget
    }

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
        // The user presses down a key with the current widget selected
    }

    fn on_key_up() 
    where Self: Sized
    {
        // The user releases a pressed key (see on_key_down)
    }
}