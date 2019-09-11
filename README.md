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
Download the above dependencies and place into project root directory
### Linux
Check for distribution-specific development packages/repos for the required libraries
### General
1. When using text, place the .ttf font in /res/font/
2. In `font.rs`, update `default_font` for the `FontParams` struct

## TODOs:
- Refactor widget ids from being `u32`s to `&'static str`s
  - Allow for lookup by string
  - Enforce uniqueness
  - Maintain in hash table (id -> widget)
- Implement ECS for widgets and views (see widgets/widget.rs & views/view.rs)
  - What must each widget share?
  - What must each view share?
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
- Documentation:
  - Create script for running `rustdoc` or `cargo doc` and generating html for RustUI crate only. Then open this html file.
  - Bash script for Linux & batch script for Windows
- Build Script:
  - Extract SDL2 libs for Windows
  - Generate docs (and provide option to open)
  - Run script
  - Make these optional via flags
- Add a GridView macro with respective methods (like tkinter)
- Implement `font.rs` (separate widgets, backend, and font).
- Accept widget ids as strings, then store their hashes?
- Store a view's widget ids in a hash table
- Stabilize FPS (without fixed loop dealys) -- implement delta time
- Implement graphics
  - Image rendering
    - Eventually, consider SVG rendering
  - Use images for buttons
    - How to handle varying width/height?
- Run callbacks on separate threads
- At some point, should be able to simply pass a canvas and render UI to *existing* canvas
- Support multiple windows