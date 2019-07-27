/*

Graphical functionality such as:
-Windows
-Texture Drawing

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
        // TODO: Flesh this out and utilize appropriately
        pub struct ApplicationState<'a, T> {
            pub hovering: Option<u32>, // Widget being hovered over
            pub clicking: Option<u32>, // Widget being clicked (left mouse down)
            pub state: &'a mut T,      // User state to be passed to widgets
        }

        impl<'a, T> ApplicationState<'a, T> {
            pub fn new(state: &'a mut T) -> Self {
                ApplicationState {
                    hovering: None,
                    clicking: None,
                    state: state,
                }
            }

            pub fn get_state(&mut self) -> &mut T {
                self.state
            }
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
        use crate::view::{View};
        use crate::widgets::WidgetState;
        use super::state::{ApplicationState};
        
        // Expected lifetime ('a) -> the initializing function containing the .start() call
        // Generic type (T) -> The user-defined application state struct for use with callbacks
        pub struct Window<'a, T> {
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
        impl<'a, T> Window<'a, T> {
            pub fn init(window_title: &str, state: &'a mut T) -> Self {
                let sdl_context = sdl2::init().map_err(|e| e.to_string()).unwrap();
                let video_subsystem = sdl_context.video().map_err(|e| e.to_string()).unwrap();
                let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

                let default_window = video_subsystem.window(window_title, 800, 600).position_centered().build().unwrap();
                let default_window_canvas = default_window.into_canvas().accelerated().build().unwrap();
                let default_window_event_pump = sdl_context.event_pump().unwrap();

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

            fn resize_window(&mut self, width: u32, height: u32) {
                self.canvas.window_mut().set_size(width, height).expect("Failed to resize");
            }

            // TODO: Allow multiple windows to run at once on multiple threads
            // TODO: How to handle window size changes from the user?
            pub fn start(mut self, mut view: View<T>) {
                // Set initial window size (will override the default of 800x600)
                self.resize_window(view.view_width, view.view_height);

                'window_loop: loop {
                    self.canvas.set_draw_color(Color::RGB(50, 50, 100));
                    self.canvas.clear();

                    'event_pump: for event in self.event_pump.poll_iter() {
                        match event {
                            Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                break 'window_loop;
                            }

                            // TODO: Making event handling widget-specific might simplify the entire idea of backend state

                            Event::MouseMotion { x, y, .. } => {
                                let event_location = Point::new(x, y);

                                self.window_state.hovering = None;

                                for widget in &view.subview {
                                    if widget.rect().contains_point(event_location) {
                                        if let Some(active_id) = self.window_state.clicking {
                                            if active_id == widget.id() {
                                                break; // Hovering over already active widget
                                            }
                                        }
                                        // Hovering over inactive widget -> set it as hover
                                        self.window_state.hovering = Some(widget.id());
                                    }
                                }
                            }

                            Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                                let event_location = Point::new(x, y);

                                self.window_state.clicking = None;
                                for widget in &view.subview {
                                    if widget.rect().contains_point(event_location) {
                                        if let Some(hover_id) = self.window_state.hovering {
                                            if hover_id == widget.id() {
                                                self.window_state.hovering = None; // Cannot be both hover & active
                                            }
                                        }
                                        self.window_state.clicking = Some(widget.id());
                                        break; // Found a widget, don't need to keep checking
                                    }
                                }
                            }

                            Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
                                let event_location = Point::new(x, y);
                                if let Some(active_id) = self.window_state.clicking { // If there is an active widget
                                    // TODO: Replace the for loop with hash table lookup (should be part of the view)
                                    for widget in &mut view.subview { // Look at each widget
                                        if widget.rect().contains_point(event_location) { // If the mouse was released on any widget
                                            if active_id == widget.id() { // Trigger the callback if that widget was active
                                                widget.on_click(self.window_state.get_state());
                                            }
                                            self.window_state.hovering = Some(widget.id()); // If the mouse is on a widget, it is now hovering
                                        }
                                        self.window_state.clicking = None; // Mouse was released, so nothing should be active
                                    }
                                }
                            }

                            // All unhandled events match here
                            _ => {
                                println!("Unhandled Event: {:?}", event);
                            }
                        }
                    }
                    // Render window below

                    // Render each widget
                    for widget in &mut view.subview {
                        widget.update(self.window_state.state);

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

                        widget.render(&mut self, widget_state);
                    }

                    self.canvas.present();

                    // FIXME: Hard-limit to 60fps to avoid excessive rendering (lowers GPU usage by 80%)
                    ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
                } // end event loop
            } // end start() method
        } // end imple window
    } // end mod window
} // end mod system
