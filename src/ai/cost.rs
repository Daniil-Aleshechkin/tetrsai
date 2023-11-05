use crate::{tetris::board::{BOARD_HEIGHT, BOARD_WIDTH, Board}, enums::piece_type::PieceType};

pub fn board_height(board: Board) -> usize {
    let mut maxHeight = 0;
    
    for x in 0..BOARD_WIDTH {
        let mut height = 0;
        for y in 0..BOARD_HEIGHT {
            if board[y][x] != PieceType::None {
                height = y;
                break;
            }
        }

        if height > maxHeight {
            maxHeight = height;
        }
    }

    maxHeight
}