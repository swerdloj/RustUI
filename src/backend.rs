/*

Graphical functionality such as:
-Windows
-Texture Drawing

TODO: How will this be implemented so the user never needs to access SDL?
Idea: Handle low-level, sdl2-related stuff here, then implement the rest elsewhere to avoid clutter
This module should therefore handle: windows, drawing to canvas

Handle events elsewhere

TODO: Should be able to support multiple windows at once
*/

extern crate sdl2;

// use sdl2::rect::Rect;

// TODO: Event loop and state management

// State Management:
// - Widget/View locations & properties
// - Widget/View layering/ordering (relative layouts?)
// - Basically just a list of items and their locations/properties

// Event Loop:
// - Select the active widget (or none or default widget)
// - Listen for relative events (e.g. clicking a button will make it active & trigger its events)
// - Perform callbacks on separate threads (or async?)
// Do this after the state management/render system is in place

// TODO: Call this 'context' instead of 'system'?
pub mod system {
    // TODO: Is this right? Then the user can implement custom state structs for their application
    pub mod state {
        pub struct ApplicationState<'a, T> {
            pub hovering: Option<u32>, // Widget being hovered over
            pub clicking: Option<u32>, // Widget being clicked (left mouse down)
            state: &'a mut T,
        }

        // TODO: Consider bringing back the State trait if needed

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
            ttf_context: sdl2::ttf::Sdl2TtfContext,
            video_subsystem: sdl2::VideoSubsystem,
            // window: sdl2::video::Window,
            canvas: sdl2::render::WindowCanvas,
            event_pump: sdl2::EventPump,

            //TODO: Is this the best way to handle state? Shouldn't it be shared across multiple windows, etc?
            window_state: ApplicationState<'a, T>,
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

            // TODO: Allow multiple windows to run at once on multiple threads
            // TODO: How to handle window size changing?
            pub fn start(mut self, view: View<T>) {
                self.canvas.set_draw_color(Color::RGB(50, 50, 100));
                self.canvas.clear();
                self.canvas.present();

                'window_loop: loop {
                    'pump: for event in self.event_pump.poll_iter() {
                        match event {
                            Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                break 'window_loop;
                            }

                            Event::MouseMotion { x, y, .. } => {
                                let event_location = Point::new(x, y);

                                self.window_state.hovering = None;

                                for widget in &view {
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
                                for widget in &view {
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
                                    for widget in &view { // Look at each widget
                                        if widget.rect().contains_point(event_location) { // If the mouse was released on any widget
                                            if active_id == widget.id() { // Trigger the callback if that widget was active
                                                widget.on_click(self.window_state.get_state());
                                            }
                                            // TODO: This logic won't work for anything other than buttons
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
                    for widget in &view {
                        if let Some(active_id) = self.window_state.clicking {
                            if active_id == widget.id() {
                                widget.render(&mut self.canvas, WidgetState::Active);
                                continue;
                            }
                        }

                        if let Some(hover_id) = self.window_state.hovering {
                            if hover_id == widget.id() {
                                widget.render(&mut self.canvas, WidgetState::Hovering);
                                continue;
                            }
                        }

                        widget.render(&mut self.canvas, WidgetState::Base);
                    }

                    self.canvas.present();

                    // FIXME: Hard-limit to 60fps to avoid excessive rendering (lowers GPU usage by 80%)
                    ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
                }
            }
        }
    }
}
