use crate::enums::{piece_type::{PieceType, PositionMap}, rotation::Rotation};

pub const BOARD_HEIGHT: usize = 23;
pub const BOARD_WIDTH: usize = 10;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub x : usize, 
    pub y : usize
}

impl Position {
    pub fn from_tuple(tuple: (usize, usize)) -> Position {
        Position { x: tuple.0, y: tuple.1 }
    }

    pub fn down(&self, value: usize) -> Position {
        Position { x: self.x, y: self.y + value }
    }

    pub fn up(&self, value: usize) -> Position {
        Position { x: self.x, y: self.y - value }
    }

    pub fn left(&self, value: usize) -> Position {
        Position { x: self.x - 1, y: self.y + value }
    }

    pub fn right(&self, value: usize) -> Position {
        Position { x: self.x + 1, y: self.y + value }
    }

    pub fn down_one(&self) -> Position {
        Position { x: self.x, y: self.y + 1 }
    }

    pub fn up_one(&self) -> Position {
        Position { x: self.x, y: self.y - 1 }
    }

    pub fn right_one(&self) -> Position {
        Position { x: self.x + 1, y: self.y }
    }

    pub fn left_one(&self) -> Position {
        Position { x: self.x - 1, y: self.y + 1 }
    }

    pub fn fits_in_board(&self) -> bool {
        self.x >= 0 && self.x < BOARD_WIDTH && self.y >= 0 && self.y < BOARD_HEIGHT
    }

    pub fn add(&self, position: Position) -> Position{
        Position { x: self.x, y: self.y }
    }
}

pub type Board = [[PieceType; 10]; 23];

pub fn get_lowest_piece_board_pos (board: Board, position: Position, piece: PieceType, rotation: Rotation) -> Option<usize> {
    for i in position.y..BOARD_HEIGHT {
        if can_piece_fit_in_location(board, position.down(i), piece, rotation) {
            return Some(i);
        }
    }

    None
}

pub fn can_piece_fit_in_location(board: Board, position: Position, piece: PieceType, rotation: Rotation) -> bool {
    match piece.get_position_map(rotation) {
        Some(offsets) => {
            for offset in offsets {
                let minoPos = position.add(offset);

                if !minoPos.fits_in_board() {
                    return false;
                }

                if board[minoPos.y][minoPos.x] != PieceType::None {
                    return false;
                }
            }

            true
        },
        None => false
    }
}

pub fn get_piece_starting_pos(piece: PieceType) -> Position {
    match piece {
        PieceType::I => Position { x: 2, y: 0 },
        PieceType::T |
        PieceType::J |
        PieceType::L |
        PieceType::S |
        PieceType::Z => Position { x: 3, y: 0 },
        PieceType::O => Position { x: 4, y: 0 },
        PieceType::None => Position { x: 0, y: 0 },
    }
}