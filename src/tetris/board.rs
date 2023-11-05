use crate::enums::{piece_type::{PieceType, PositionMap}, rotation::Rotation};

pub const BOARD_HEIGHT: usize = 23;
pub const BOARD_WIDTH: usize = 10;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Position {
    pub x : i32, 
    pub y : i32
}

impl Position {
    pub fn from_tuple(tuple: (i32, i32)) -> Position {
        Position { x: tuple.0, y: tuple.1 }
    }

    pub fn down(&self, value: i32) -> Position {
        Position { x: self.x, y: self.y + value }
    }

    #[allow(dead_code)]
    pub fn up(&self, value: i32) -> Position {
        Position { x: self.x, y: self.y - value }
    }

    pub fn left(&self, value: i32) -> Position {
        Position { x: self.x - value, y: self.y}
    }

    pub fn right(&self, value: i32) -> Position {
        Position { x: self.x + value, y: self.y }
    }

    pub fn fits_in_board(&self) -> bool {
        self.x < BOARD_WIDTH as i32 && self.y < BOARD_HEIGHT as i32 && self.x >= 0 && self.y >= 0
    }

    pub fn add(&self, position: Position) -> Position{
        Position { x: self.x + position.x, y: self.y + position.y }
    }
}

pub type Board = [[PieceType; BOARD_WIDTH]; BOARD_HEIGHT];

pub fn get_lowest_piece_board_pos (board: Board, position: Position, piece: PieceType, rotation: Rotation) -> Option<i32> {
    for i in 0..BOARD_HEIGHT {
        if !can_piece_fit_in_location(board, position.down(i as i32), piece, rotation) {
            return Some(i as i32 - 1 + position.y);
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

                if board[minoPos.y as usize][minoPos.x as usize] != PieceType::None {
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