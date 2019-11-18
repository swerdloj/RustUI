extern crate sdl2;
use sdl2::surface::Surface;
use sdl2::pixels::{PixelFormat, PixelFormatEnum};

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
            // TODO: This shouldn't be case sensitive
            match os_str.to_str() {
                None => {
                    // FIXME: This should never be reacehd
                    Err("No extension was found".to_owned())
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

// TODO: It is probably better to just obtain a u8 array directly from the file rather than
//        doing all this FFI stuff and working with 2 surfaces
fn extend_surface(surface: &Surface /*, (width, height) */) -> Result<(/* Surface */), String> {

    // FIXME: There is no easy way to convert pixel formats. See above TODO instead.

    // let RGBA8888_surface = surface.convert(&PixelFormat::from(PixelFormatEnum::RGBA8888));
    // let pixel_format = surface.pixel_format_enum();

    unsafe {
        // Handle to raw SDL_Surface
        let raw_surface = *surface.raw();

        // Total number of pixels
        let num_pixels = (raw_surface.w * raw_surface.h) as isize;

        // Represent pixel data as unsigned 8-bit values
        let pixels = raw_surface.pixels as *mut u8;

        // Note that range is exclusive on [0, n)
        for i in 0..num_pixels {
            // WARNING: incorrect offsets introduce undefined behavior (does not crash program)
            if i % 4 == 0 {
                *pixels.offset(i) = 0;
            }
        }
    }
    
    // temporary
    Err("Failed".to_owned())
}