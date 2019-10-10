/*

Application backend -- techinacal aspects the user should never need to see


TODO: Should events be handled by widgets? This would allow for specific callbacks:
For example, the user could utilize text input when the enter key is pressed.

TODO: Should be able to support multiple windows at once.
This will likely require user state to be guarded by a mutex/semaphore.
Each window will run on its own thread.

*/

// TODO: Consider moving event handling to Widget functionality

extern crate sdl2;


// TODO: Call this 'context' instead of 'system'?
pub mod system {    
    pub mod state {
        use crate::view_components::views::view::View;

        // TODO: Flesh this out and utilize appropriately. Or move event handling to Widget
        /// Holds application state relating to window events
        /// - `hovering`: Mouse is hovering over widget
        /// - `clicking`: Left mouse button is pressed over widget
        /// - `focused`: The widget currently focused on (e.g.: `TextBox`)
        pub struct ApplicationState<'a, T: GenerateView<T, T>> {
            /// Widget being hovered
            pub hovering: Option<&'static str>,
            /// Widget being clicked (mouse down)
            pub clicking: Option<&'static str>,
            /// Focused Widget (maintains active state after mouse up)
            pub focused: Option<&'static str>,
            /// User state to be passed to widgets
            pub user_state: &'a mut T,
        }

        impl<'a, T: GenerateView<T, T>> ApplicationState<'a, T> {
            pub fn new(user_state: &'a mut T) -> Self {
                ApplicationState {
                    hovering: None,
                    clicking: None,
                    focused: None,
                    user_state: user_state,
                }
            }
        }

        // TODO: Is here the correct place for this trait?
        // FIXME: Box is a workaround
        // FIXME: Can this work with only one generic param?
        pub trait GenerateView<T, S> {
            fn generate_view(&self) -> Box<dyn View<S>>;
        }
    } // end mod state

    /// This module handles application windows and related events:
    /// - Window Creation
    /// - Event Handling (within the window)
    /// - Application State (both backend and user-level)
    pub mod window {
        use sdl2::pixels::Color;
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        use sdl2::mouse::MouseButton;
        use sdl2::rect::Point;
        use crate::view_components::widgets::widget::WidgetState;
        use crate::font::{FontParams, Fonts};
        use super::state::{ApplicationState, GenerateView};
        
        // Expected lifetime ('a) -> the initializing function containing the .start() call
        // Generic type (T) -> The user-defined application state struct for use with callbacks
        pub struct Window<'a, T: GenerateView<T, T>> {
            sdl_context: sdl2::Sdl,
            pub ttf_context: sdl2::ttf::Sdl2TtfContext,
            video_subsystem: sdl2::VideoSubsystem,
            // window: sdl2::video::Window,
            
            pub canvas: sdl2::render::WindowCanvas,
            event_pump: sdl2::EventPump,

            //TODO: Is this the best way to handle state? Shouldn't it be shared across multiple windows, etc?
            pub window_state: ApplicationState<'a, T>,
        }

        // TODO: Create a builder similar to widget declaration
        //       include things like .scale, .resizable, .accelerated, .background_color, etc.
        impl<'a, T: GenerateView<T, T> + Clone + PartialEq> Window<'a, T> {
            pub fn init(window_title: &str, state: &'a mut T) -> Self {
                let sdl_context = sdl2::init().map_err(|e| e.to_string()).unwrap();
                let video_subsystem = sdl_context.video().map_err(|e| e.to_string()).unwrap();
                let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

                let mut default_window = video_subsystem
                                     .window(window_title, 800, 600)
                                     .position_centered()
                                     .build()
                                     .expect("Failed to create window");

                // Raise and focus the window
                // FIXME: Move this somewhere else and remove above `mut`
                default_window.raise();

                let default_window_canvas = default_window
                                            .into_canvas()
                                            .accelerated()
                                            .build()
                                            .expect("Failed to create window canvas");

                let default_window_event_pump = sdl_context
                                                .event_pump()
                                                .expect("Failed to obtain event pump");

                Window {
                    sdl_context: sdl_context,
                    ttf_context: ttf_context,
                    video_subsystem: video_subsystem,
                    // window: default_window,
                    canvas: default_window_canvas,
                    event_pump: default_window_event_pump,
                    window_state: ApplicationState::new(state),
                }
            }

            /// Set the window icon to the specified image
            pub fn set_icon(&mut self, resource_path: &str) {

                // FIXME: This is temporary. See TODO below
                let path = std::path::Path::new(resource_path);
                let surface = sdl2::surface::Surface::load_bmp(path).expect("Failed to load resource");

                let mut window = self.canvas.window_mut();

                window.set_icon(surface);

                /*  TODO:
                    1. Convert str to Path
                    2. Match on path ending (.jpg, .bmp, .png, etc.)
                    3. Load surface
                    4. Consider creating a generic 'load surface' function that takes str and returns surface
                    
                    For now, use the sdl2::surface::Surface::load_bmp(path) method
                */

                // It looks like a `SurfaceRef` is simply a `&Surface`
                // window.set_icon(SurfaceRef)
            }

            /// Used for scaling
            // TODO: See this: https://gamedev.stackexchange.com/questions/119414/resolution-scaling
            pub fn set_logical_size(&mut self, width: u32, height: u32) {
                self.canvas.set_logical_size(width, height).unwrap();
            }

            /// Resizes the application window to the specified pixel values
            /// Usage: `resize_window((width, height));`
            fn resize_window(&mut self, dimensions: (u32, u32)) {
                self.canvas.window_mut().set_size(dimensions.0, dimensions.1).expect("Failed to resize");
            }

            // TODO: Allow multiple windows to run at once on multiple threads
            // TODO: How to handle window size changes from the user?
            // pub fn start<V: View<T> + Sized>(mut self, mut view: V) {
            pub fn start(mut self) {
                /* Initialize here */

                // Used to determine whether to resize window
                let mut last_window_size = (0u32, 0u32);
                // Used to detect state changes, triggering view generation
                let mut last_user_state = self.window_state.user_state.clone();
                // Stores the root view
                let mut view = self.window_state.user_state.generate_view();

                // Initialize the window/widget layout
                view.init(&self.ttf_context);
                // FIXME: This is only needed because only the parent
                //        view should call this explicitly
                view.align();
                // FIXME: This needs to account for nested views if not fixed_size
                // Set initial window size (will override the default of 800x600)
                self.resize_window(view.view_size());

                'window_loop: loop {
                    // Only update the view tree if state was modified
                    if *self.window_state.user_state != last_user_state {
                        last_user_state = self.window_state.user_state.clone();

                        // Generate the new view
                        view = self.window_state.user_state.generate_view();
                        view.init(&self.ttf_context);
                        view.align();

                        // View's size has changed -> adjust
                        if view.view_size() != last_window_size {
                            last_window_size = view.view_size();
                            self.resize_window(last_window_size);
                        }
                    }

                    self.canvas.set_draw_color(Color::RGB(50, 50, 100));
                    self.canvas.clear();

                    'event_pump: for event in self.event_pump.poll_iter() {
                        match event {
                            Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                break 'window_loop;
                            }

                            // TODO: Making event handling widget-specific might simplify the entire idea of backend state

                            // Determine hover state
                            Event::MouseMotion { x, y, .. } => {
                                let event_location = Point::new(x, y);

                                self.window_state.hovering = None;

                                for widget in view.child_widgets_mut() {
                                    if widget.rect().contains_point(event_location) {
                                        if let Some(active_id) = self.window_state.clicking {
                                            if active_id == widget.id() {
                                                break; // Hovering over already active widget
                                            }
                                        }
                                        // Hovering over inactive widget -> set it as hover
                                        self.window_state.hovering = Some(widget.id());
                                        break; // don't need to check other widgets
                                    }
                                }
                            }

                            Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                                let event_location = Point::new(x, y);

                                // Whether a widget was clicked
                                let mut clicked_widget = false;

                                self.window_state.clicking = None;
                                for widget in view.child_widgets_mut() {
                                    if widget.rect().contains_point(event_location) {
                                        clicked_widget = true;

                                        if let Some(focus_id) = self.window_state.focused {
                                            if focus_id != widget.id() { // Did not click previously active widget
                                                self.window_state.focused = None; // That widget is no longer active
                                            }
                                        }

                                        if let Some(hover_id) = self.window_state.hovering {
                                            if hover_id == widget.id() {
                                                self.window_state.hovering = None; // Cannot be both hover & active
                                            }
                                        }
                                        self.window_state.clicking = Some(widget.id());
                                        break; // Found a widget, don't need to keep checking
                                    }
                                }

                                // If no widgets were clicked, no widgets should be focused
                                if !clicked_widget {
                                    self.window_state.focused = None;
                                }
                            }

                            Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
                                let event_location = Point::new(x, y);
                                if let Some(active_id) = self.window_state.clicking { // If there is an active widget
                                    // TODO: Replace the for loop with hash table lookup (should be part of the view)
                                    for widget in view.child_widgets_mut() { // Look at each widget
                                        if widget.rect().contains_point(event_location) { // If the mouse was released on any widget
                                            if active_id == widget.id() { // Trigger the callback if that widget was active
                                                widget.on_click(self.window_state.user_state);

                                                //FIXME: Clicking nothing should also defocus
                                                if widget.can_focus() {
                                                    self.window_state.focused = Some(widget.id());
                                                }
                                            }
                                            self.window_state.hovering = Some(widget.id()); // If the mouse is on a widget, it is now hovering
                                            break; // Already handled click. Can stop checking for collision.
                                        }
                                    }
                                    self.window_state.clicking = None; // Mouse was released, so nothing should be active
                                }
                            }

                            // All unhandled events match here
                            _ => {
                                // println!("Unhandled Event: {:?}", event);
                            }
                        }

                        // Update focused widget
                        if let Some(focus_id) = self.window_state.focused { // find widget if one is focused
                            for widget in view.child_widgets_mut() {
                                if focus_id == widget.id() {
                                    widget.update(self.window_state.user_state, &event);
                                    break; // found widget, don't need to keep looking
                                }
                            }
                        }
                    } // end event loop

                    /* Render window below this line */

                    // Render each widget
                    for widget in view.child_widgets_mut() {
                        let mut widget_state = WidgetState::Base;

                        if let Some(active_id) = self.window_state.clicking {
                            if active_id == widget.id() {
                                widget_state = WidgetState::Active;
                            }
                        }

                        if let Some(hover_id) = self.window_state.hovering {
                            if hover_id == widget.id() {
                                widget_state = WidgetState::Hovering;
                            }
                        }

                        if let Some(focus_id) = self.window_state.focused {
                            if focus_id == widget.id() {
                                // FIXME: Need to update widget before rendering
                                widget_state = WidgetState::Focused;
                            }
                        }

                        widget.render(&mut self, widget_state);
                    }

                    self.canvas.present();

                    // FIXME: Replace this with delta time for use in animations & frame rate limiting
                    // Hard-limit to 60fps to avoid excessive rendering (lowers GPU usage considerably)
                    ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
                } // end event loop
            } // end start() method
        } // end impl window
    } // end mod window
} // end mod system
