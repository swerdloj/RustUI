/*

Handle font loading, storage, and sizing

*/

use sdl2::ttf;

enum Font {
    // (path, size)
    FontParams(Path, u16),
}

pub struct Fonts {
    // TODO: Map of Font enums to loaded ttf::Font
    fonts: Vec<Font::FontParams>,
}

impl Fonts {
    pub fn get_font(path: &str, size: u16) -> ttf::Font
    where &str: Into<Path> {

    }
}