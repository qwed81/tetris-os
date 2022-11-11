use crate::kernel::graphics::{ TTYFrame, Color };
use core::fmt::Write;
use crate::kernel::interrupts::{ Key, KeyboardState };
use super::{ BlockList, BlockState, BOARD_HEIGHT, BOARD_WIDTH };

impl TTYFrame {
    
    pub fn render_outline(&mut self) {
        self.draw_line_verticale(0, 0, BOARD_HEIGHT, Color::DarkGray);
        self.draw_line_verticale(1, 0, BOARD_HEIGHT, Color::DarkGray);

        self.draw_line_verticale(22, 0, BOARD_HEIGHT, Color::DarkGray);
        self.draw_line_verticale(23, 0, BOARD_HEIGHT, Color::DarkGray);

        self.draw_line_horizontal(0, 0, 21, Color::DarkGray);
        self.draw_line_horizontal(BOARD_HEIGHT + 1, 0, 23, Color::DarkGray);
    }

    pub fn render_stale(&mut self, board: &[[BlockState; BOARD_HEIGHT]; BOARD_WIDTH]) {
        for i in 0..BOARD_WIDTH {
            for j in 0..BOARD_HEIGHT {
                if let BlockState::Block(color) = board[i][j] {
                    self.render_block(i, j, color);
                }
            }
        }
    }

    pub fn print_end_screen(&mut self, lines: u64) {
        core::write!(self, "game over, {} lines cleared", lines).unwrap();
    }

    pub fn print_score(&mut self, lines: u64) {
        core::write!(self, "lines cleares: {}", lines).unwrap();
    }

    pub fn print_keyboard_state(&mut self, current_time: u64, last_time: u64, keyboard: &KeyboardState) {
        core::write!(self, "time is: {}, last is: {}\n", current_time, last_time).unwrap();
        core::write!(self, "up arrow is down: {}\n", keyboard.is_key_down(Key::UpArrow)).unwrap();    
        core::write!(self, "down arrow is down: {}\n", keyboard.is_key_down(Key::DownArrow)).unwrap();    
        core::write!(self, "left arrow is down: {}\n", keyboard.is_key_down(Key::LeftArrow)).unwrap();    
        core::write!(self, "right arrow is down: {}\n", keyboard.is_key_down(Key::RightArrow)).unwrap();    
        core::write!(self, "space is down: {}", keyboard.is_key_down(Key::Space)).unwrap();    
    }

    pub fn render_shape(&mut self, color: Color, 
        list: &BlockList) {

        for (x, y) in list {
            self.render_block(*x, *y, color);
        }
    }

    // render block with their logical game position
    fn render_block(&mut self, x: usize, y: usize, color: Color) {
        self.draw_line_horizontal(y + 1, x * 2 + 2, x * 2 + 3, color);
    }

}




