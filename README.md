# RustUI
Rust GUI library inspired by SwiftUI.

## Testing Binaries
Different executables can be tested by including them in /src/bin.  
Run them via `cargo run --bin file_name`

## Current Standing
Very early development

## Dependencies
The following items are required to run RustUI projects:
- [SDL2](https://www.libsdl.org/download-2.0.php)
  - SDL2.dll
  - SDL2.lib
- [SDL2_tff](https://www.libsdl.org/projects/SDL_ttf/)
  - SDL2_tff.dll
  - SDL2_tff.lib
  - libfreetype-6.dll
  - zlib1.dll

## Usage (unstable)
1. Download the above dependencies, and place into project root directory
2. When using text, place the .ttf font in /res/font/
3. In widgets.rs, update the Text widget's render function with the new font file

### TODOs:
- Basic Widgets:
  - Slider (horizontal + vertical)
  - Text Input (static + dynamic)
  - Drop Menu
  - Radio Buttons
- Basic Views:
  - HStack
  - Menubar
  - Header/Footer
- Basic Components:
  - Dividers
  - Nested fixed-size views scrollable via scrollbars
- Add a GridView macro with respective methods (like tkinter)
- Implement `font.rs` (separate widgets, backend, and font).
- Accept widget ids as strings, then store their hashes
- Store a view's widget ids in a hash table
- Stabilize FPS (without fixed loop dealys) -- implement delta time
- Implement graphics
- Run callbacks on separate threads
- At some point, should be able to simply pass a canvas and render UI to *existing* canvas
- Support multiple windows