use super::{BOARD_WIDTH, BOARD_HEIGHT, BlockState, BlockType, BlockRotation, BlockList, Board};

// gives you back either the list of all the positions,
// or none in the case it is out of bounds, or intersecting another
// block
pub fn list(board: &Board, x: usize, y: usize, 
    block_type: BlockType, r: BlockRotation) -> Option<BlockList> {

    if y >= super::BOARD_HEIGHT || x >= super::BOARD_WIDTH {
        return None;
    }

    let piece_positions = match block_type {
        BlockType::Square => position_list_square(x, y),
        BlockType::T => position_list_t(x, y, r),
        BlockType::S => position_list_s(x, y, r),
        BlockType::I => position_list_i(x, y, r),
        BlockType::Z => position_list_z(x, y, r),
        BlockType::J => position_list_j(x, y, r),
        BlockType::L => position_list_l(x, y, r),
    }?;

    // filter's out positions that overlap something else
    for (x, y) in piece_positions {
        if let BlockState::Block(_) = board[x][y] {
            return None;
        }
    }

    Some(piece_positions)
}

fn position_list_j(x: usize, y: usize, r: BlockRotation) -> Option<BlockList> {
    use BlockRotation::*;
    match r {
        Zero => {
            if y + 1 >= BOARD_HEIGHT || x + 1 >= BOARD_WIDTH {
                return None;
            }
            Some([(x, y), (x + 1, y - 1), (x, y + 1), (x, y - 1)])
        },
        Single => {
            if y + 1 >= BOARD_HEIGHT || x == 0 || x + 1 >= BOARD_WIDTH {
                return None;
            }
            Some([(x, y), (x - 1, y), (x + 1, y), (x + 1, y + 1)])
        },
        Double => {
            if y + 1 >= BOARD_HEIGHT || x == 0 {
                return None;
            }
            Some([(x, y), (x - 1, y + 1), (x, y + 1), (x, y - 1)])

        },
        Triple => {
            if x == 0 {
                return None;
            }
            Some([(x, y), (x - 1, y), (x + 1, y), (x - 1, y - 1)])
        }
    }
}

fn position_list_l(x: usize, y: usize, r: BlockRotation) -> Option<BlockList> {
    use BlockRotation::*;
    match r {
        Zero => {
            if y + 1 >= BOARD_HEIGHT || x + 1 >= BOARD_WIDTH {
                return None;
            }
            Some([(x, y), (x, y - 1), (x, y + 1), (x + 1, y + 1)])

        },
        Single => {
            if x == 0 || x + 1 == BOARD_WIDTH && y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x - 1, y), (x - 1, y + 1), (x + 1, y)])
        },
        Double => {
            if x == 0 || y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x - 1, y - 1), (x, y - 1), (x, y + 1)])
        },
        Triple => {
            if x == 0 || x + 1 >= BOARD_WIDTH {
                return None;
            }
            Some([(x, y), (x + 1, y - 1), (x - 1, y), (x + 1, y)])
        }
    }
}

fn position_list_i(x: usize, y: usize, r: BlockRotation) -> Option<BlockList> {
    use BlockRotation::*;
    match r {
        Zero | Double => {
            if y + 3 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]) 
        },
        Single | Triple => {
            if x == 0 || x + 2 >= BOARD_WIDTH {
                return None;
            }
            Some([(x, y), (x - 1, y), (x + 1, y), (x + 2, y)])
        }
    }
}


fn position_list_s(x: usize, y: usize, r: BlockRotation) -> Option<BlockList> {
    use BlockRotation::*;
    match r {
        Zero | Double => {
            if x == 0 || x + 1 >= BOARD_WIDTH || y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x, y + 1), (x - 1, y + 1), (x + 1, y)])
        }
        Single | Triple => {
            if x + 1 >= BOARD_WIDTH || y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x + 1, y), (x, y - 1), (x + 1, y + 1)])
        }
    }
}

fn position_list_z(x: usize, y: usize, r: BlockRotation) -> Option<BlockList> {
    use BlockRotation::*;
    match r {
        Zero | Double => {
            if x == 0 || x + 1 >= BOARD_WIDTH || y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x - 1, y), (x, y + 1), (x + 1, y + 1)])
        }, 
        Single | Triple => {
            if x + 1 >= BOARD_WIDTH || y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x + 1, y), (x + 1, y - 1), (x, y + 1)])
        }
    }
}

fn position_list_t(x: usize, y: usize, r: BlockRotation) -> Option<BlockList> {
    use BlockRotation::*;
    match r {
        Zero => {
            if x == 0 || x + 1 >= BOARD_WIDTH || y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x - 1, y), (x + 1, y), (x, y + 1)])
        },
        Single => {
            if x == 0 || y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x - 1, y), (x, y - 1), (x, y + 1)])
        },
        Double => {
            if x == 0 || x + 1 >= BOARD_WIDTH {
                return None;
            }
            Some([(x, y), (x, y - 1), (x - 1, y), (x + 1, y)])
        },
        Triple => {
            if x + 1 >= BOARD_WIDTH || y + 1 >= BOARD_HEIGHT {
                return None;
            }
            Some([(x, y), (x + 1, y), (x, y - 1), (x, y + 1)])
        }
    }
}


fn position_list_square(x: usize, y: usize) -> Option<BlockList> {
    if x + 1 >= BOARD_WIDTH || y + 1 >= BOARD_HEIGHT {
        return None;
    }
    Some([(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)])
}
