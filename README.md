# RustUI
Rust GUI library inspired by SwiftUI

## Testing Binaries
Different executables can be tested by including them in src/bin.  
Run them via `cargo run --bin file_name`

## Current Standing
Setup phase

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

### TODOs:
- [ ] Implement widgets
- [ ] Accept widget ids as strings, then store their hashes
- [ ] Store a view's widget ids in a hash table
- [x] Iterate through all active widgets in event loop
- [ ] Allow for Python tkinter-style callbacks
- [ ] Implement text
- [ ] Stabilize FPS
- [ ] Implement graphics
- [ ] Run callbacks on separate threads
- [ ] At some point, should be able to simply pass a canvas and render UI to *existing* canvas