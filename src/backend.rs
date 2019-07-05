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
            video_subsystem: sdl2::VideoSubsystem,
            // window: sdl2::video::Window,
            canvas: sdl2::render::WindowCanvas,
            event_pump: sdl2::EventPump,
        }

        impl Window {
            pub fn init(window_title: &str) -> Self {
                let context = sdl2::init().unwrap();
                let video_subsystem = context.video().unwrap();

                let default_window = video_subsystem.window(window_title, 800, 600).position_centered().build().unwrap();
                let default_window_canvas = default_window.into_canvas().accelerated().build().unwrap();
                let default_window_event_pump = context.event_pump().unwrap();

                Window {
                    sdl_context: context,
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

                'window_loop: loop {
                    'pump: for event in self.event_pump.poll_iter() {
                        match event {
                            Event::Quit {..} |
                            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                break 'window_loop;
                            }

                            Event::MouseButtonDown { x, y, .. } => {
                                let event_location = Point::new(x, y);
                                for widget in &view {
                                    if widget.get_rect().contains_point(event_location) {
                                        active_widget = Some(widget.get_id());
                                        break;
                                    } else {
                                        active_widget = None;
                                    }
                                }
                            }

                            Event::MouseButtonUp { x, y, .. } => {
                                let event_location = Point::new(x, y);
                                if let Some(id) = active_widget {
                                    // TODO: Replace the for loop with hash table lookup (should be part of the view)
                                    for widget in &view {
                                        if widget.get_rect().contains_point(event_location) && id == widget.get_id() {
                                            widget.on_click();
                                        }
                                    }
                                }
                            }

                            _ => {
                                println!("Unhandled Event: {:?}", event);
                            }
                        }
                    }
                    // TODO: Render window here

                    self.canvas.set_draw_color(Color::RGB(240, 240, 200));

                    for widget in &view {
                        self.canvas.fill_rect(widget.get_rect()).unwrap();
                    }

                    self.canvas.present();
                }
            }
        }
    }
}
