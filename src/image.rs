extern crate sdl2;
use sdl2::surface::Surface;

use std::path::Path;

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
                Some("jpg") |
                Some("jpeg") => {
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
                None => {
                    // TODO: Change this. Did something go wrong?
                    Err(format!("No extension was found"))
                }
                _ => {
                    // TODO: Print only file suffix?
                    Err(format!("File at {:?} is of unsupported type", path))
                }
            }
        }
        None => {
            Err(format!("File {:?} has no suffix", path))
        }
    }
}

// fn load_png(path: &Path) -> Surface {
    
// }

// fn load_jpeg(path: &Path) -> Surface {

// }

fn load_bitmap(path: &Path) -> Surface {
    Surface::load_bmp(path).expect("Failed to load bitmap")
}