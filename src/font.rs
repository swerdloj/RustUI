/*

Handle font loading, storage, and sizing

TODO: Major refactoring to include this in widgets.rs & backend.rs

*/

use sdl2::ttf;

use std::collections::HashMap;
use std::path::Path;

pub struct FontParams {
    /// Path to the font
    path: Box<Path>,
    /// Point size of the font
    point_size: u16,
}

// TODO: TTF font objects are not hashable. Consider the following structure:
//       Vec of fonts -> each font is assigned an index
//       Map the font params to the Vec[index] for that font
//       Lookup by obtaining the vec index from the font params -> maintain O(1)

pub struct Fonts<'window> {
    // TODO: Map of Font enums to loaded ttf::Font
    ttf_context: &'window ttf::Sdl2TtfContext,
    default: ttf::Font<'window, 'static>,

}

impl<'window> Fonts<'window> {
    pub fn new(ttf_context: &'window ttf::Sdl2TtfContext, path: &str, point_size: u16) -> Self {
        Fonts {
            ttf_context: ttf_context,
            default: ttf_context.load_font(Path::new(path), point_size).expect("Failed to load default font"),
        }
    }

    pub fn get_font(path: String, size: u16) {

    }

    /// Loads and stores a font for future use
    pub fn load_font(&mut self, ttf_context: &ttf::Sdl2TtfContext, path: String, size: u16) {
        let font = ttf_context.load_font(path, size).expect("Failed to load font");
        
    }
}