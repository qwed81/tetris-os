#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct GraphicBlock {
    character: u8,
    color: u8,
}

impl Default for GraphicBlock {
    fn default() -> GraphicBlock {
        GraphicBlock::solid(Color::Black)
    }
}

impl GraphicBlock {
    pub fn with_char(foreground: Color, background: Color,
        c: u8) -> GraphicBlock {
        let color = (background as u8) << 4 | (foreground as u8);
        GraphicBlock {
            color,
            character: c
        }
    }

    pub fn solid(color: Color) -> GraphicBlock {
        let color = (color as u8) << 4;
        GraphicBlock {
            color, character: b' '
        }
    }
    
}

pub const HEIGHT: usize = 25;
pub const WIDTH: usize = 80;
const VGA_BUFFER_PTR: *mut GraphicBlock = 0xb8000 as *mut GraphicBlock;
const TEXT_COLOR: Color = Color::Cyan;

pub struct TTYFrame {
    frame_data: [[GraphicBlock; WIDTH]; HEIGHT],
    row: usize,
    col: usize,
    bounds: TTYBounds
}

#[derive(Clone, Copy, Debug)]
pub struct TTYBounds {
    pub x: usize,
    pub y: usize,
    pub end_y: usize,
    pub end_x: usize
}

impl TTYFrame {
    pub fn new() -> TTYFrame {
        TTYFrame {
            frame_data: [[GraphicBlock::default(); WIDTH]; HEIGHT],
            row: 0,
            col: 0,
            bounds: TTYBounds {
                x: 0, y: 0, 
                end_x: WIDTH - 1,
                end_y: HEIGHT - 1
            }
        }
    }

    pub fn flush(&self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                // writes directly to memory mapped io
                // we can tell that this is safe, because the arithmatic never overflows isize
                unsafe {
                    let offset_ptr = VGA_BUFFER_PTR.offset((i * WIDTH + j) as isize);
                    offset_ptr.write_volatile(self.frame_data[i][j]);
                }
            }
        }
    }

    pub fn draw_line_verticale(&mut self, x: usize, mut y1: usize,
        y2: usize, color: Color) {
        
        while y1 <= y2 {
            self.draw_square(x, y1, color);
            y1 += 1;
        }

    }

    pub fn draw_line_horizontal(&mut self, y: usize, mut x1: usize,
        x2: usize, color: Color) {
       
        while x1 <= x2 {
            self.draw_square(x1, y, color);
            x1 += 1;
        }
    }

    pub fn draw_square(&mut self, x: usize, y: usize, color: Color) {
        self.frame_data[y][x] = GraphicBlock::solid(color);
    }

    pub fn draw_char(&mut self, x: usize, y: usize, c: u8) {
        self.frame_data[y][x] = GraphicBlock::with_char(TEXT_COLOR, Color::Black, c);
    }

    pub fn set_write_bounds(&mut self, bounds: TTYBounds) {
        self.bounds = bounds;
        self.row = bounds.y;
        self.col = bounds.x;
    }
}

impl core::fmt::Write for TTYFrame {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            if *c == b'\n' {
                self.row += 1;
                self.col = self.bounds.x;
                continue;
            }

            if self.row > self.bounds.end_y {
                break;
            }

            if self.col > self.bounds.end_x {
                self.col = self.bounds.x;
                self.row += 1;
            }

            self.draw_char(self.col, self.row, *c);
            self.col += 1;
        }

        Ok(())
    }
}

pub fn quick_write_message(message: &str) {
    use core::fmt::Write;
    let mut frame = TTYFrame::new();
    core::write!(&mut frame, "{}", message).unwrap();    
    frame.flush();
}
