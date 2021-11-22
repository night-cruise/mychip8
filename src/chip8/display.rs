use crate::chip8::PlatForm;

// the chip-8 uses a 64x32-pixel monochrome display with this format:
// (0,0)	       (63,0)
//
//
// (0,31)	       (63,31)
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

/// screen display
pub struct Display {
    pixels: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH], // pixel matrix
    redraw: bool,
}

impl Display {
    /// create a display instance
    pub fn new() -> Display {
        Display {
            pixels: [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
            redraw: true,
        }
    }

    /// return the screen height
    pub fn display_height() -> usize {
        DISPLAY_HEIGHT
    }

    /// set pixel on the (x, y)
    /// return a bool value to indicate whether the pixel is erased
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: bool) -> bool {
        let x = x % DISPLAY_WIDTH;
        let y = y % DISPLAY_HEIGHT;
        if self.pixels[x][y] != pixel {
            self.redraw = true;
        }
        // check if the pixel will be erased
        let pixel_erased = self.pixels[x][y] && pixel;

        self.pixels[x][y] ^= pixel;

        pixel_erased
    }

    /// clear the display
    pub fn clear(&mut self) {
        self.pixels = [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];
    }

    /// check whether to redraw
    pub fn redraw(&self) -> bool {
        self.redraw
    }

    pub fn draw(&mut self, platform: &mut PlatForm) -> Result<(), String> {
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                if self.pixels[x][y] {
                    platform.draw_pixel(x as u8, y as u8)?
                }
            }
        }
        self.redraw = false;
        Ok(())
    }
}

impl Default for Display {
    fn default() -> Display {
        Display::new()
    }
}
