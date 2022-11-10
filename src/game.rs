use crate::kernel::interrupts::{ self, Key, KeyboardState };
use crate::kernel::graphics::{ self, Color, TTYFrame, TTYBounds };
use lazy_static::lazy_static;
use spin::Mutex;

mod display;
mod block_list;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 22;

#[derive(Copy, Clone, Debug)]
pub enum BlockState {
    Block(Color), Empty
}

#[derive(Copy, Clone, Debug)]
pub enum BlockType {
    I, Z, S, T, Square, J, L
}

impl BlockType {
    fn color(self) -> Color {
        use BlockType::*;
        match self {
            I => Color::Cyan,
            Z => Color::Red,
            S => Color::Blue,
            T => Color::Magenta,
            Square => Color::Yellow,
            J => Color::Green,
            L => Color::Pink
        }
    }
}

impl BlockType {
    fn random(time: u64) -> BlockType {
        use BlockType::*;
        let result = time % 7;
        match result {
            0 => I,
            1 => Z,
            2 => S,
            3 => T,
            4 => Square,
            5 => J,
            6 => L,
            _ => panic!("Not all randoms handled")
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum BlockRotation {
    Zero, Single, Double, Triple
}

impl BlockRotation {

    fn right_rotate(self: BlockRotation) -> Self {
        use BlockRotation::*;
        match self {
            Zero => Single,
            Single => Double,
            Double => Triple,
            Triple => Zero
        }
    }
}

type Board = [[BlockState; BOARD_HEIGHT]; BOARD_WIDTH];
type BlockList = [(usize, usize); 4];

pub struct GameState {
    board: [[BlockState; BOARD_HEIGHT]; BOARD_WIDTH],
    block_x: usize,
    block_y: usize,
    block_rotation: BlockRotation,
    block_type: BlockType,
    time_of_last_move: u64,
    last_keyboard: KeyboardState,
    lines_cleared: u64,
    over: bool
}

lazy_static! {
    static ref STATE: Mutex<GameState> = Mutex::new(GameState {
        board: [[BlockState::Empty; BOARD_HEIGHT]; BOARD_WIDTH],
        block_x: 4,
        block_y: 1,
        block_rotation: BlockRotation::Zero,
        block_type: BlockType::Square,
        time_of_last_move: 0,
        last_keyboard: KeyboardState::blank(),
        lines_cleared: 0,
        over: false
    });
}

pub fn run(current_time: u64, keyboard: KeyboardState, frame: &mut TTYFrame) {
    let mut state = STATE.lock();

    if state.over {
        frame.print_end_screen(state.lines_cleared);
        return;
    }

    handle_keyboard(&mut state, &keyboard, current_time);

    // if it was moved with the keyboard,
    // then it needs to be recalculated to the new position
    let down_list = translated_list(&state, 0, 1);
    let current_list = match translated_list(&state, 0, 0) {
        Some(list) => list,
        None => {
            state.over = true;
            return;
        }
    };

    if current_time - state.time_of_last_move > 4 {
        match down_list {
            Some(_) => state.block_y += 1,
            None => solidify_piece(&mut state, &current_list, current_time / 4),
        }

        state.time_of_last_move = current_time;
    }
  
    frame.set_write_bounds(TTYBounds {
        y: 1, x: 30, end_x: graphics::WIDTH - 1,
        end_y: graphics::HEIGHT - 1
    });

    //frame.print_keyboard_state(current_time, state.time_of_last_move, &keyboard);
    frame.print_score(state.lines_cleared);
    frame.render_shape(state.block_type.color(), &current_list);
    frame.render_stale(&state.board);
    frame.render_outline();

    state.last_keyboard = keyboard;
}

// shorthand for a translation that is used a lot
fn translated_list(state: &GameState, x_trans: isize, 
    y_trans: isize) -> Option<BlockList> {

    // prevent underflow of unsigned type
    if (state.block_x as isize) + x_trans < 0 || 
        (state.block_y as isize) + y_trans < 0 {

        return None;
    }

    let x = ((state.block_x as isize) + x_trans) as usize;
    let y = ((state.block_y as isize) + y_trans) as usize;
    block_list::list(&state.board, x, y, state.block_type, state.block_rotation)
}

fn handle_keyboard(state: &mut GameState, keyboard: &KeyboardState, time: u64) {
    // move down on hold, even if pressed before
    if keyboard.is_key_down(Key::DownArrow) &&
        state.last_keyboard.is_key_down(Key::DownArrow) {

        if let Some(_) = translated_list(state, 0, 1) {
            state.block_y += 1;
        }
    }

    // handle all the other ones, only if an actual event occured
    if state.last_keyboard.input_version() != keyboard.input_version() { 
        if keyboard.is_key_down(Key::Space) {
            while let Some(_) = translated_list(state, 0, 1) {
                state.block_y += 1;
            }
            solidify_piece(state, &translated_list(state, 0, 0).unwrap(), time);
        }
        if keyboard.is_key_down(Key::LeftArrow) {
            if let Some(_) = translated_list(state, -1, 0) {
                state.block_x -= 1;
            }
        }
        if keyboard.is_key_down(Key::RightArrow) { 
            if let Some(_) = translated_list(state, 1, 0) {
                state.block_x += 1;
            }
        }
        if keyboard.is_key_down(Key::UpArrow) {
            let rotation = state.block_rotation.right_rotate();
            if let Some(_) = block_list::list(&state.board, state.block_x,
                state.block_y, state.block_type, rotation) {
                state.block_rotation = rotation;
            }
        }
        
    }
    
}

fn solidify_piece(state: &mut GameState, list: &BlockList, time: u64) {
    for (x, y) in list {
        state.board[*x][*y] = BlockState::Block(state.block_type.color());
    }

    state.block_y = 1; // to avoid overflow
    state.block_x = 5; 
    state.block_type = BlockType::random(time);
    state.block_rotation = BlockRotation::Zero;
    
    // check if lines need to be cleared
    let mut line_clear_amt = 0;
    let mut line_clear_end = 0;
    for j in 0..BOARD_HEIGHT {
        let mut all_covered = true;
        for i in 0..BOARD_WIDTH {
            if let BlockState::Empty = state.board[i][j] {
                all_covered = false;
            }
        }

        if all_covered {
            line_clear_amt += 1;
            line_clear_end = j;
        }
    }

    // clear lines if necessary
    for j in (0..=line_clear_end).rev() {
        for i in 0..BOARD_WIDTH {
            if j < line_clear_amt {
                state.board[i][j] = BlockState::Empty;
            }
            else {
                state.board[i][j] = state.board[i][j - line_clear_amt];
            }
        }
    }

    state.lines_cleared += line_clear_amt as u64;

}


