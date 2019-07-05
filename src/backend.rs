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
use super::view;

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
    /// This module handles application windows and related events:
    /// - Window Creation
    /// - Event Handling (within the window)
    pub mod window {
        use sdl2::pixels::Color;
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;
        use sdl2::rect::Point;
        use super::super::view::{View};
        
        pub struct Window {
            sdl_context: sdl2::Sdl,
            ttf_context: sdl2::ttf::Sdl2TtfContext,
            video_subsystem: sdl2::VideoSubsystem,
            // window: sdl2::video::Window,
            canvas: sdl2::render::WindowCanvas,
            event_pump: sdl2::EventPump,
        }

        impl Window {
            pub fn init(window_title: &str) -> Self {
                let sdl_context = sdl2::init().unwrap();
                let video_subsystem = sdl_context.video().unwrap();

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
                }
            }

            // TODO: Allow multiple windows to run at once on multiple threads
            // TODO: How to handle window size changing?
            pub fn start(mut self, view: View) {
                self.canvas.set_draw_color(Color::RGB(50, 50, 100));
                self.canvas.clear();
                self.canvas.present();

                let mut active_widget: Option<u32> = None;
                let mut hover_widget: Option<u32> = None;

                'window_loop: loop {
                    'pump: for event in self.event_pump.poll_iter() {
                        match event {
                            Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                break 'window_loop;
                            }

                            Event::MouseMotion { x, y, .. } => {
                                let event_location = Point::new(x, y);

                                hover_widget = None;

                                for widget in &view {
                                    if widget.get_rect().contains_point(event_location) {
                                        if let Some(active_id) = active_widget {
                                            if active_id == widget.get_id() {
                                                break; // Hovering over already active widget
                                            }
                                        } else {
                                            hover_widget = Some(widget.get_id());
                                        }
                                    }
                                }
                            }

                            Event::MouseButtonDown { x, y, .. } => {
                                let event_location = Point::new(x, y);

                                active_widget = None;

                                for widget in &view {
                                    if widget.get_rect().contains_point(event_location) {
                                        if let Some(hover_id) = hover_widget {
                                            if hover_id == widget.get_id() {
                                                hover_widget = None; // Cannot be both hover & active
                                            }
                                        }
                                        active_widget = Some(widget.get_id());
                                        break; // Found a widget, don't need to keep checking
                                    }
                                }
                            }

                            Event::MouseButtonUp { x, y, .. } => {
                                let event_location = Point::new(x, y);
                                if let Some(active_id) = active_widget {
                                    // TODO: Replace the for loop with hash table lookup (should be part of the view)
                                    for widget in &view {
                                        if widget.get_rect().contains_point(event_location) && active_id == widget.get_id() {
                                            widget.on_click();
                                            // TODO: This logic won't work for anything other than buttons
                                            hover_widget = active_widget; // no longer active, now hovering
                                        } 
                                        active_widget = None;
                                    }
                                }

                                // FIXME: This is an expensive workaround for a bug:
                                //        When releasing the mouse on some widgets, they are no longer active,
                                //        but they are not assigned as the hover widget for some reason (above)
                                for widget in &view {
                                    if widget.get_rect().contains_point(event_location) {
                                        hover_widget = Some(widget.get_id());
                                        break;
                                    }
                                }
                            }

                            // All unhandled events match here
                            _ => {
                                // println!("Unhandled Event: {:?}", event);
                            }
                        }
                    }
                    // Render window below

                    for widget in &view {
                        // Default to primary
                        self.canvas.set_draw_color(widget.primary_color());

                        if let Some(active_id) = active_widget {
                            if active_id == widget.get_id() {
                                // println!("A widget is active");
                                self.canvas.set_draw_color(widget.secondary_color());
                            }
                        }

                        if let Some(hover_id) = hover_widget {
                            if hover_id == widget.get_id() {
                                // println!("The mouse is hovering over a widget");
                                self.canvas.set_draw_color(widget.hover_color());
                            }
                        }

                        self.canvas.fill_rect(widget.get_rect()).unwrap();
                    }

                    self.canvas.present();

                    // FIXME: Hard-limit to 60fps to avoid excessive rendering (lowers GPU usage by 80%)
                    ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
                }
            }
        }
    }
}
