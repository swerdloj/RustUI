extern crate sdl2;
use sdl2::surface::Surface;

use std::path::Path;

/* TODO: Consider allowing transparent color keys for bitmaps (and others?):

See https://docs.rs/sdl2/0.32.2/sdl2/surface/struct.SurfaceRef.html#method.set_color_key
and https://wiki.libsdl.org/SDL_SetColorKey

*/

// TODO: Error enum (e.g.: Unsupported, LoadFailed, etc.)


/// Load an image from the specified path. 
/// ### Supports:
/// - jpeg (`.jpg`, `.jpeg`)
/// - png (`.png`)
/// - bitmap (`.bmp`)
pub fn load_image(path: &Path) -> Result<Surface, String> {
    return match path.extension() {
        Some(os_str) => {
            match os_str.to_str() {
                None => {
                    // FIXME: This should never be reacehd
                    Err(format!("No extension was found"))
                }

                Some("jpg") | Some("jpeg") => {
                    // Ok(load_jpeg(&path))
                    Err("Not implemented".to_owned())
                }

                Some("bmp") => {
                    Ok(load_bitmap(&path))
                }

                Some("png") => {
                    // Ok(load_png(&path))
                    Err("Not implemented".to_owned())
                }

                _ => {
                    // TODO: Print only file suffix?
                    Err(format!("File '{:?}' has unsupported extension, '{:?}'", path, os_str))
                }
            }
        }

        None => { // No extension
            Err(format!("File '{:?}' has no extension", path))
        }
    }
}

// fn load_png(path: &Path) -> Surface {
    
// }

// fn load_jpeg(path: &Path) -> Surface {

// }

// TODO: Implement
fn load_bitmap(path: &Path) -> Surface {
    Surface::load_bmp(path).expect("Failed to load bitmap")
}