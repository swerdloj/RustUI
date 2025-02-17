# RustUI
Rust GUI library inspired by SwiftUI.

This library emphasises the usage of declarative syntax for clean & simple UI development.  

[Read the project writeup](https://github.com/swerdloj/RustUI/blob/master/writeup.pdf)

![](demo.gif)

## Disclaimer
This project was created as a means of exploring declarative UI syntax.  
RustUI is **not** intended for real-world use.  
I created RustUI to learn Rust, SDL2, and to explore UI libraries. Much refactoring is needed for further development.

## Examples
See [testing.rs](https://github.com/swerdloj/RustUI/blob/master/src/bin/testing.rs) for library capabilities and syntax.  
Note that you will need a custom bitmap image to replace both instances of "temp_logo_low_quality.bmp" (simply change that string to the new image path).

## Testing Binaries
Different executables can be tested by including them in /src/bin.  
Run them via `cargo run --bin file_name`  
For simplification, a script can be created to enter this command and any parameters.

## Current Standing
Early development

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
Download the above dependencies and place into project root directory (with `Cargo.toml`)
### Linux
Check for distribution-specific development packages/repos for the required libraries
### General
1. When using text, place the .ttf font in /res/font/
2. In `font.rs`, update `default_font` for the `FontParams` struct

## TODOs:
- See [Projects](https://github.com/swerdloj/RustUI/projects)
- Font Management:
  - When a font is needed, query font map
  - if not found, load font and store for future use
  - 90% of this library's CPU usage is font loading each frame (caused by Rust's SDL2 library lifetime usage)
- Persistent State:
  - Persist widget state between view-generation cycles
    - User would no longer need to maintain that widget's state
  - Consider a map of widget id -> widget data
    - Enforces unique ids
  - Can use a trait to get data that needs to persist
    - How to get this data back from the map?
- Persistent State Alternative:
  - Do not generate new view, instead update view and re-layout
  - Implement a signal for view change to generate new view
  - Would then only lose state when new view is obtained
- Implement ECS for widgets and views (see widgets/widget.rs & views/view.rs)
  - What must each widget share?
  - What must each view share?
  - What do both share?
- Scaling for high resolution/dpi displays
- Better GFX:
  - Rounded rects
  - Circles
  - Cursors
- Basic Widgets:
  - Drop Menu
  - Radio Buttons
- Basic Views:
  - Menubar
  - Header/Footer
- Basic Components:
  - Dividers
  - Nested fixed-size views scrollable via scrollbars
- Basic Utilities:
  - File Browser
  - Message/Dialog Boxes
- Documentation:
  - Finish documenting and add examples
- Build Script:
  - Extract SDL2 libs for Windows
  - Generate docs (and provide option to open)
  - Run script
  - Make these optional via flags
- Add a GridView macro with respective methods (like tkinter)
- Implement `font.rs` (separate widgets, backend, and font).
- Stabilize FPS (without fixed loop dealys) -- implement delta time
- Implement graphics
  - Image rendering
    - Eventually, consider SVG rendering
  - Use images for buttons
    - How to handle varying width/height?
- Run callbacks on separate threads
- At some point, should be able to simply pass a canvas and render UI to *existing* canvas
  - This will require a different render cycle, as the user will define the run-loop
- Support multiple windows