/*

Handle font loading, storage, and sizing

TODO: Major refactoring to include this in widgets.rs & backend.rs

*/

use sdl2::ttf;
use sdl2::ttf::Sdl2TtfContext;

use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;

// ========================== Font Parameters ========================== //

// Eq -> iff self.fields == other.fields
#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct FontParams {
    /// Path to the font
    pub path: &'static str,
    /// Point size of the font
    pub point_size: u16,
}

impl FontParams {
    pub fn new(path: &'static str, point_size: u16) -> Self {
        FontParams {
            path: path,
            point_size: point_size,
        }
    }

    pub fn default_font() -> Self {
        FontParams {
            path: "./res/font/OpenSans-Regular.ttf",
            point_size: 20,
        }
    }
}

// ========================== Font Backend ========================== //

/// Backend for font handling
pub struct Fonts<'ttf> {
    // The TTF context utilized by the application for text
    // ttf_context: &'ttf Sdl2TtfContext,

    /// Map of (FontParams -> Loaded Font)
    font_map: HashMap<FontParams, ttf::Font<'ttf, 'static>>,
}

impl<'ttf> Fonts<'ttf> {
    pub fn new(/*ttf_context: &'ttf Sdl2TtfContext */) -> Self {
        Fonts {
            // ttf_context: ttf_context,
            font_map: HashMap::new(),
        }
    }

    /// Obtain a reference to the desired font
    // TODO: How to handle missing font? Return default? Load and store that font?
    pub fn get_font(&self, font: &FontParams) -> &ttf::Font {
        self.font_map.get(&font).expect("No such font exists")
    }

    /// Load and store a font for future use
    pub fn load_font(&mut self, ttf_context: &'ttf Sdl2TtfContext, font_params: &FontParams) {
        // TODO: assert that the font is not already loaded (saves some execution time)
        let font = ttf_context.load_font(font_params.path, font_params.point_size).expect("Failed to load font");
        self.font_map.insert(*font_params, font);
    }

    pub fn render_surface(&mut self, font: &FontParams, text: &str, color: sdl2::pixels::Color) -> Box<sdl2::surface::Surface> {
        let surface = self.font_map.get(font)
            .expect("No such font exists")
            .render(text)
            .blended(color)
            .expect("Failed to render surface");

        Box::new(surface)
    }

    /// Obtain a string's surface parameters (width, height) for a font without rendering
    pub fn size_surface(&self, font: &FontParams, text: &str) -> (u32, u32) {
        self.get_font(font).size_of(text).expect("Failed to query text size")
    }
}