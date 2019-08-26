# RustUI
Rust GUI library inspired by SwiftUI.

## Testing Binaries
Different executables can be tested by including them in /src/bin.  
Run them via `cargo run --bin file_name`  
For simplification, a script can be created to enter this command and any parameters.

## Current Standing
Very early development

## Dependencies
The following items are required to run RustUI projects:
- [SDL2](https://www.libsdl.org/download-2.0.php)
  - SDL2
- [SDL2_ttf](https://www.libsdl.org/projects/SDL_ttf/)
  - SDL2_ttf
  - libfreetype-6
  - zlib1

## Usage (unstable)
### Windows
Download the above dependencies, and place into project root directory
### Linux
Check for distribution-specific packages/repos
### General
1. When using text, place the .ttf font in /res/font/
2. In font.rs, update `default_font` for the `FontParams` struct

### TODOs:
- Scaling for high resolution/dpi displays
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