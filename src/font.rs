/*

Handle font loading, storage, and sizing

TODO: Major refactoring to include this in widgets.rs & backend.rs

*/

use sdl2::ttf;

use std::collections::HashMap;
use std::path::Path;

// ========================== Font Parameters ========================== //

// Eq -> iff self.fields == other.fields
#[derive(PartialEq, Eq, Hash)]
pub struct FontParams {
    /// Path to the font
    path: &'static Path,
    /// Point size of the font
    point_size: u16,
}

impl FontParams {
    pub fn new(path: &'static str, point_size: u16) -> Self {
        FontParams {
            path: Path::new(path),
            point_size: point_size,
        }
    }

    pub fn default_font() -> Self {
        FontParams {
            path: Path::new("./res/font/OpenSans-Regular.ttf"),
            point_size: 20,
        }
    }
}

// ========================== Font Backend ========================== //

/// Backend for font handling
pub struct Fonts<'window> {
    /// The TTF context utilized by the application for text
    ttf_context: &'window ttf::Sdl2TtfContext,

    /// Default font, guarenteed to be loaded and ready for use
    default: ttf::Font<'window, 'static>,

    /// Map of (FontParams -> Loaded Font)
    font_map: HashMap<FontParams, ttf::Font<'window, 'static>>,
}

impl<'window> Fonts<'window> {
    pub fn new(ttf_context: &'window ttf::Sdl2TtfContext, default_font_path: &str, point_size: u16) -> Self {
        Fonts {
            ttf_context: ttf_context,
            default: ttf_context.load_font(Path::new(default_font_path), point_size).expect("Failed to load default font"),
            // fonts: Vec::new(),
            font_map: HashMap::new(),
        }
    }

    /// Obtain a reference to the desired font
    // TODO: How to handle missing font? Return default? Load and store that font?
    pub fn get_font(&self, font: FontParams) -> &ttf::Font {
        self.font_map.get(&font).expect("No such font exists")
    }

    /// Load and store a font for future use
    pub fn load_font(&mut self, path: &'static str, point_size: u16) {
        let font = self.ttf_context.load_font(path, point_size).expect("Failed to load font");
        self.font_map.insert(FontParams::new(path, point_size), font);
    }

    /// Obtain a string's surface parameters (width, height) for a font without rendering
    pub fn size_surface(&self, font: FontParams, text: &str) -> (u32, u32) {
        self.get_font(font).size_of(text).expect("Failed to query text size")
    }
}