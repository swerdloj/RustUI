/*

Widget functionality

Jonathan Swerdlow
7-2-19

TODO: As the project grows, consider splitting individual widgets into their own files
 and instead use this as a 'widget backend' of sorts.

TODO: Consider moving event handling (backend) to be part of Widget functionality

TODO: To properly fit view components, there must be some way of getting text rect dimensions
 from a string given font size.
 Consider a ViewComponent trait with methods such as `get_width` and `get_height`
 and implement this for all widgets and views
*/

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::pixels::Color;
use crate::backend::system::window::Window;
use crate::view::{ViewComponent, ViewComponentType};
use crate::font;

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

/// Default color implementations
pub mod colors {
    use super::Color;

    /// `Color::RGB(0, 0, 0)` - Black
    pub const BLACK: Color = Color {r: 0, g: 0, b: 0, a: 0xff};
    /// `Color::RGB(50, 50, 100)` - Default background color
    pub const DARK_PURPLE: Color = Color {r: 50, g: 50, b: 100, a: 0xff};
    /// `Color::RGB(240, 240, 200)` - Default button color
    pub const MANILLA: Color = Color {r: 240, g: 240, b: 200, a: 0xff};
    /// `Color::RGB(255, 255, 255)` - White
    pub const WHITE: Color = Color {r: 255, g: 255, b: 255, a: 0xff};
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

// TODO: Replace 'T' with 'S' for the sake of clarity?
// NOTE: In this module, the generic type 'T' refers EXCLUSIVELY to user-defined state

// TODO: Modify `with_text` builder methods to accept full text widgets
// This will enable the user to customize widget text without redundant methods

// ========================== BUTTON WIDGET ========================== //

pub struct Button<T> {
    pub id: u32,
    pub rect: Rect,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub hover_color: Color,
    pub text: Option<Text<T>>,
    pub on_click: Option<Box<Fn(&mut T)>>,
}

impl<T> Button<T> {
    pub fn new(id: &str) -> Self {
        Button {
            id: 0,
            rect: Rect::new(0, 0, 100, 40),
            primary_color: colors::MANILLA,
            secondary_color: Color::RGB(100, 100, 100),
            hover_color: Color::RGB(200, 200, 200),
            text: None,
            on_click: None,
        }
    }

    pub fn with_text(mut self, text: &str) -> Self {
        // TODO: How to hanle the sub-widget's id?
        //       Note that the sub-widget is not actually part of the view
        self.text = Some(
            Text::new("", text)
            .center()
        );
        self
    }

    pub fn with_on_click<F: 'static + Fn(&mut T)>
    (mut self, callback: F) -> Self
    {
        self.on_click = Some(Box::new(callback));
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
            button_text.container_rect = self.rect;
        }

        self
    }
}

impl<T> Widget<T> for Button<T> {
    fn text_component(&mut self) -> Option<&mut Text<T>> {
        if let Some(text) = &mut self.text {
            return Some(text);
        }
        None
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        self.text_component().expect("Attempted to size nonexistant text component").assign_text_dimensions(dims);
    }

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

    fn translate(&mut self, dx: i32, dy: i32) {
        self.rect = Rect::new(
            self.rect().x() + dx,
            self.rect().y() + dy,
            self.rect().width(),
            self.rect().height()
        );

        if let Some(button_text) = &mut self.text {
            button_text.translate(dx, dy);
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

    fn update(&mut self, state: &T) {

    }

    fn on_click(&mut self, state: &mut T) {
        if let Some(ref on_click_function) = self.on_click {
            (on_click_function)(state);
        }
    }

    fn draw_width(&self) -> u32 {
        self.rect.width()
    }
}

// ========================== TEXT WIDGET ========================== //

// See https://github.com/Rust-SDL2/rust-sdl2/blob/master/src/sdl2/ttf/font.rs for help
// TTF is undocumented on sdl2 crate docs.

pub struct Text<T> {
    id: u32,
    container_rect: Rect,
    primary_color: Color,
    pub text: String,
    pub font: font::FontParams,
    // How far text must be from its boundary
    internal_padding: u32,

    update_fn: Option<Box<Fn(&T) -> String>>,

    auto_resize: bool,
    center_text: bool,
   
    // Text surface parameters    
    text_width: u32,
    text_height: u32,
}

impl<T> Text<T> {
    pub fn new(id: &str, text: &str) -> Self {
        Text {
            id: 100,
            container_rect: Rect::new(0, 0, 0, 28), // FIXME: 28 is only true for default font
            primary_color: colors::BLACK,
            text: String::from(text),
            font: font::FontParams::default_font(),
            internal_padding: 10,
            update_fn: None,
            auto_resize: false,
            center_text: false,
            text_width: 0,
            text_height: 0,
        }
    }

    pub fn center(mut self) -> Self {
        self.center_text = true;
        self
    }

    pub fn with_text_update<F: 'static + Fn(&T) -> String>
    (mut self, update_fn: F) -> Self  
    {
        self.update_fn = Some(Box::new(update_fn));
        self
    }

    // TODO: id should be hashed from new(str), delete this later
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.primary_color = color;
        self
    }

    pub fn with_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.primary_color = Color::RGB(r, g, b);
        self
    }

    pub fn with_rgba(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.primary_color = Color::RGBA(r, g, b, a);
        self
    }

    pub fn auto_resize(mut self) -> Self {
        self.auto_resize = true;
        self
    }

    pub fn place(mut self, x: i32, y: i32) -> Self {
        self.container_rect = Rect::new(x, y, self.container_rect.width(), self.container_rect.height());
        self
    }

    // TODO: This only rescales the text in one dimension. It should rescale both dimensions by the same factor
    // TODO: Update this with padding_left, padding_right, padding_top, padding_bottom when implemented
    fn fit_and_center_within_container(&self, container_rect: &Rect) -> Rect {
        let width_constraint = (container_rect.width() as i32 - self.internal_padding as i32) as u32;
        let height_constraint = (container_rect.height() as i32 - self.internal_padding as i32) as u32;
        
        // Determine whether the text is too tall and/or too wide
        let width_ratio = self.text_width as f32 / width_constraint as f32;
        let height_ratio = self.text_height as f32 / height_constraint as f32;

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
            (self.text_width, self.text_height)
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

impl<T> Widget<T> for Text<T> {
    fn text_component(&mut self) -> Option<&mut Text<T>> {
        Some(self)
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        self.text_width = dims.0;
        self.text_height = dims.1;
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.container_rect = Rect::new(
            self.rect().x() + dx,
            self.rect().y() + dy,
            self.rect().width(),
            self.rect().height()
        );
    }

    fn rect(&self) -> Rect {
        self.container_rect
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

    fn on_click(&mut self, state: &mut T) {

    }

    // TODO: Resize the text surface on update
    fn update(&mut self, state: &T) {
        if let Some(ref update_callback) = self.update_fn {
            self.text = (update_callback)(state);
        }
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

        let texture = texture_creator.create_texture_from_surface(surface).expect("Failed to create texture");
        // let TextureQuery { width, height, .. } = texture.query();

        // println!("Rendering '{}' with containter {}x{} at ({}, {}) and text size {}x{}", self.text, self.container_rect.width(), self.container_rect.height(), self.container_rect.x(), self.container_rect.y(), self.text_width, self.text_height);

        let target = if self.auto_resize {
            // Center text within container & downscale if too large
            self.fit_and_center_within_container(&self.container_rect)
        } else if self.center_text {
            // Center the text, disregarding containers
            let center_x = self.container_rect.x() + self.container_rect.width() as i32 / 2;
            let target_x = center_x - self.text_width as i32 / 2;
            Rect::new(
                target_x,
                self.container_rect.y() + ((self.container_rect.height() as i32 - self.text_height as i32) / 2),
                self.text_width,
                self.text_height
            )
        } else {
            // Center the text's y position and align left
            Rect::new(
                self.container_rect.x(),
                self.container_rect.y() + ((self.container_rect.height() as i32 - self.text_height as i32) / 2),
                self.text_width,
                self.text_height
            )
        };

        window.canvas.copy(&texture, None, Some(target)).unwrap();
    }

    fn draw_width(&self) -> u32 {
        self.text_width
    }
}

// ========================== CHECKBOX WIDGET ========================== //

pub struct CheckBox<T> {
    id: u32,
    rect: Rect,
    default_color: Color,
    click_color: Color,
    hover_color: Color,
    check_color: Color,
    text: Option<Text<T>>,
    is_checked: bool,
    
    checkbox_padding_right: u32,
    // Callback accepting application state & check state
    on_check: Option<Box<Fn(&mut T, bool)>>,

    checkbox_width: u32,
    checkbox_height: u32,
}

impl<T> CheckBox<T> {
    pub fn new(id: &str) -> Self {
        CheckBox {
            id: 200,
            rect: Rect::new(0, 0, 100, 40),
            default_color: colors::MANILLA,
            click_color: Color::RGB(140, 140, 140),
            hover_color: Color::RGB(200, 200, 200),
            check_color: Color::RGB(80, 80, 80),
            text: None,
            is_checked: false,
            checkbox_padding_right: 10,
            on_check: None,

            checkbox_width: 20,
            checkbox_height: 20,
        }
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn with_text(mut self, text: &str) -> Self {
        // TODO: How to hanle the sub-widget's id?
        //       Note that the sub-widget is not actually part of the view
        self.text = Some(
            Text::new("", text)
                .place(self.rect.x() + (self.checkbox_width + self.checkbox_padding_right) as i32, 
                       self.rect.y()
                )
                .with_color(colors::WHITE)
        );
        self
    }

    pub fn place(mut self, x: i32, y: i32) -> Self {
        self.rect = Rect::new(x, y, self.rect.width(), self.rect.height());

        if let Some(text) = &mut self.text {
            text.container_rect = Rect::new(
                x + (self.checkbox_width + self.checkbox_padding_right) as i32,
                // TODO: Is this y position correct?
                self.rect.y(),
                self.rect.width(),
                self.rect.height()
            );
        }

        self
    }

    pub fn with_on_check<F: 'static + Fn(&mut T, bool)>
    (mut self, check_fn: F) -> Self {
        self.on_check = Some(Box::new(check_fn));
        self
    }

    pub fn check(mut self) -> Self {
        self.is_checked = true;
        self
    }

    pub fn on_check(&mut self, state: &mut T) {
        self.is_checked = !self.is_checked;

        if let Some(on_check_fn) = &self.on_check {
            (on_check_fn)(state, self.is_checked);
        }
    }

}

impl<T> Widget<T> for CheckBox<T> {
    fn text_component(&mut self) -> Option<&mut Text<T>> {
        if let Some(text) = &mut self.text {
            return Some(text);
        }
        None
    }

    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {
        self.rect.set_width(self.checkbox_width + self.checkbox_padding_right + dims.0);
        self.text_component().expect("Attempted to size nonexistant text component").assign_text_dimensions(dims);
    }

    fn rect(&self) -> Rect {
        self.rect
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn primary_color(&self) -> Color {
        self.default_color
    }

    fn secondary_color(&self) -> Color {
        self.click_color
    }

    fn hover_color(&self) -> Color {
        self.hover_color
    }

    // TODO: Allow user to decide whether this should trigger when the widget is clicked
    //       or *only* when the checkbox itself is clicked (would need x/y coords)
    fn on_click(&mut self, state: &mut T) {
        self.on_check(state);
    }

    // TODO: Should the text also change color on hover? such as making it slightly gray?
    fn render(&self, window: &mut Window<T>, widget_state: WidgetState) {
        // First, draw the checkbox itself
        match widget_state {
            WidgetState::Hovering => window.canvas.set_draw_color(self.hover_color),
            WidgetState::Active => window.canvas.set_draw_color(self.click_color),
            WidgetState::Base => window.canvas.set_draw_color(self.default_color),
        }
        let checkbox_x = self.rect.x();// + self.internal_padding as i32;
        let checkbox_y = self.rect.y() + (self.rect.height() as i32 - self.checkbox_height as i32) / 2;

        window.canvas.fill_rect(Rect::new(checkbox_x, checkbox_y, self.checkbox_width, self.checkbox_height)).unwrap();
    
        // Second, draw the check if checked
        if self.is_checked {
            window.canvas.set_draw_color(self.check_color);
            window.canvas.fill_rect(Rect::new(
                checkbox_x + 4, 
                checkbox_y + 4, 
                self.checkbox_width - 8, 
                self.checkbox_height - 8)
            ).unwrap();
        }

        // Finally, draw the text if present
        if let Some(text) = &self.text {
            text.render(window, widget_state);
        }
    }

    fn update(&mut self, state: &T) {

    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.rect = Rect::new(
            self.rect.x() + dx,
            self.rect.y() + dy,
            self.rect.width(),
            self.rect.height()
        );

        if let Some(text) = &mut self.text {
            text.translate(dx, dy);
        }
    }

    fn draw_width(&self) -> u32 {
        self.rect().width()
    }
}


// ========================== WIDGET TRAIT ========================== //

pub trait Widget<T> {
    fn rect(&self) -> Rect;
    fn id(&self) -> u32;
    fn primary_color(&self) -> Color;
    fn secondary_color(&self) -> Color;
    fn hover_color(&self) -> Color;

    fn text_component(&mut self) -> Option<&mut Text<T>>;

    /// Update the widget with known text dimensions  
    /// - Note that this function is called **only when text exists**  
    /// - Improper usage will therefore `panic` at `.expect()` on a `None` object
    fn assign_text_dimensions(&mut self, dims: (u32, u32)) {

    }

    // fn place(&mut self, x: i32, y: i32);

    fn on_click(&mut self, state: &mut T);

    /// Render the widget to the window
    fn render(&self, window: &mut Window<T>, widget_state: WidgetState);

    /// Update the widget according to state
    fn update(&mut self, state: &T);

    /// Translate the widget by the given x & y differences
    fn translate(&mut self, dx: i32, dy: i32);
    
    /// The widget's rendered width including any containers and sub-objects
    fn draw_width(&self) -> u32;

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

// ========================== Macro Helper Trait Implementations ========================== //

impl<T> ViewComponent for Button<T> {
    fn get_component_type(&self) -> ViewComponentType {
        ViewComponentType::Widget
    }
}

impl<T> ViewComponent for Text<T> {
    fn get_component_type(&self) -> ViewComponentType {
        ViewComponentType::Widget
    }
}

impl<T> ViewComponent for CheckBox<T> {
    fn get_component_type(&self) -> ViewComponentType {
        ViewComponentType::Widget
    }
}