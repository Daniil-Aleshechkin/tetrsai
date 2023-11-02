use std::iter::repeat;
use std::collections::VecDeque;

use crate::enums::piece_type::PositionMap;
use crate::enums::{piece_type::PieceType, rotation::Rotation};
use super::queue::Fill;
use super::board::{Board, Position, get_piece_starting_pos, get_lowest_piece_board_pos, BOARD_HEIGHT, BOARD_WIDTH, can_piece_fit_in_location};

#[derive(Copy, Clone, PartialEq)]
pub struct TetrisGameState {
    pub currentPieceType: PieceType,
    pub currentPieceRotation: Rotation,
    pub currentPieceLocation: Position,
    pub holdPieceType: PieceType,
    pub boardState: Board,
    pub previewQueue: [PieceType; 5],
    pub isLose: bool,
    pub linesSent: usize,
    pub combo: usize,
    pub backToBack: usize,
    pub hasHeld: bool,
}

pub fn hard_drop(initialState: TetrisGameState, queue: Option<&mut VecDeque<PieceType>>) -> TetrisGameState {

    if initialState.currentPieceType == PieceType::None || initialState.isLose {
        return initialState;
    }

    let mut newState: TetrisGameState = initialState;

    let nextPiece = initialState.previewQueue[0];

    newState.previewQueue = match queue {
        Some(q) => {
            q.pop_front();
            q.fill_bag();
            q.iter().take(5).cloned().collect::<Vec<_>>().into_iter().chain(repeat(PieceType::None)).take(5).collect::<Vec<_>>()[..5].try_into().expect("Guarenteed to be 5 elements here")
        },
        None => {
            let mut next = [PieceType::default(); 5];
            for (i, p) in initialState.previewQueue[1..5].iter().enumerate() {
                next[i] = p.clone();
            }
            next
        },
    };

    newState.currentPieceLocation = get_piece_starting_pos(nextPiece);
    newState.currentPieceRotation = Rotation::None;
    newState.currentPieceType = nextPiece;

    let mut isLoseFromCurrentPiece = false;
    for offset in newState.currentPieceType.get_position_map(newState.currentPieceRotation).expect("Guard exists for None") {
        let xPos = offset.x + newState.currentPieceLocation.x;
        let yPos = offset.y + newState.currentPieceLocation.y;
        
        if initialState.boardState[yPos as usize][xPos as usize] != PieceType::None {
            isLoseFromCurrentPiece = true;
            break;
        }
    }

    let hardDropLocation = get_lowest_piece_board_pos(
                                        initialState.boardState,
                                     initialState.currentPieceLocation,
                                        initialState.currentPieceType, 
                                     initialState.currentPieceRotation).expect("No y location found for hard drop");

    let mut isLoseFromPlacement = true;
    for offset in initialState.currentPieceType.get_position_map(initialState.currentPieceRotation).expect("Guard exists for None") {  
        let xPos = offset.x + initialState.currentPieceLocation.x;
        let yPos = offset.y + hardDropLocation;

        if yPos >= 3 && isLoseFromPlacement {
            isLoseFromPlacement = false;
        }

        newState.boardState[yPos as usize][xPos as usize] = initialState.currentPieceType;
    }

    let mut cornorCount = 0;

    for corner in [Position::from_tuple((0,0)), Position::from_tuple((0,2)), Position::from_tuple((2,0)), Position::from_tuple((2,2))] {
        let xPos = corner.x + initialState.currentPieceLocation.x;
        let yPos = corner.y + hardDropLocation;

        if yPos >= BOARD_HEIGHT as i32 || xPos >= BOARD_WIDTH as i32 || yPos < 0 || xPos < 0 {
            continue;
        }

        if (newState.boardState[yPos as usize][xPos as usize] != PieceType::None) {
            cornorCount += 1;
        }    
    }

    let isTSpin = initialState.currentPieceType == PieceType::T && cornorCount >= 3;
    
    let nextBoardNonClearedLines = newState.boardState.iter().cloned().filter(|&row| !is_full_row(row)).collect::<Vec<_>>();

    let linesCleared = BOARD_HEIGHT - nextBoardNonClearedLines.len(); 

    newState.boardState = repeat([PieceType::default(); 10]).take(linesCleared).chain(nextBoardNonClearedLines.into_iter()).collect::<Vec<_>>().try_into().expect("linescleared + filtered lines should board height");

    if (linesCleared == 4 || isTSpin) {
        newState.backToBack += 1;
    } else {
        newState.backToBack = 0;
    }

    if (linesCleared != 0) {
        newState.combo += 1;
    } else {
        newState.combo = 0;
    }

    newState.linesSent = caculateLinesSent(linesCleared, isTSpin, initialState.combo, initialState.backToBack);

    newState.isLose = isLoseFromCurrentPiece || isLoseFromPlacement;
    newState.hasHeld = false;
    newState
}

pub fn soft_drop(initialState: TetrisGameState) -> TetrisGameState {
    let mut newState = initialState;

    newState.currentPieceLocation.y = get_lowest_piece_board_pos(initialState.boardState, initialState.currentPieceLocation, initialState.currentPieceType, initialState.currentPieceRotation).expect("No y position for soft drop");

    newState
}

pub fn move_right(initialState: TetrisGameState) -> TetrisGameState {
    let mut newState = initialState;

    if !can_piece_fit_in_location(initialState.boardState, initialState.currentPieceLocation.right(1), initialState.currentPieceType, initialState.currentPieceRotation) {
        return newState;
    }

    newState.currentPieceLocation = initialState.currentPieceLocation.right(1);

    newState
}

pub fn move_left(initialState: TetrisGameState) -> TetrisGameState {
    let mut newState = initialState;

    if !can_piece_fit_in_location(initialState.boardState, initialState.currentPieceLocation.left(1), initialState.currentPieceType, initialState.currentPieceRotation) {
        return newState;
    }

    newState.currentPieceLocation = initialState.currentPieceLocation.left(1);

    newState
}

pub fn rotate_90(initialState: TetrisGameState) -> TetrisGameState {
    let mut newState = initialState;

    if !can_piece_fit_in_location(initialState.boardState, initialState.currentPieceLocation, initialState.currentPieceType, initialState.currentPieceRotation + Rotation::Clock) {
        return newState;
    }

    newState.currentPieceRotation = initialState.currentPieceRotation + Rotation::Clock;

    newState
}

pub fn rotate_180(initialState: TetrisGameState) -> TetrisGameState {
    let mut newState = initialState;

    if !can_piece_fit_in_location(initialState.boardState, initialState.currentPieceLocation, initialState.currentPieceType, initialState.currentPieceRotation + Rotation::OneEighty) {
        return newState;
    }

    newState.currentPieceRotation = initialState.currentPieceRotation + Rotation::OneEighty;

    newState
}

pub fn rotate_270(initialState: TetrisGameState) -> TetrisGameState {
    let mut newState = initialState;

    if !can_piece_fit_in_location(initialState.boardState, initialState.currentPieceLocation, initialState.currentPieceType, initialState.currentPieceRotation + Rotation::Counter) {
        return newState;
    }

    newState.currentPieceRotation = initialState.currentPieceRotation + Rotation::Counter;

    newState
}

pub fn hold_piece(initialState: TetrisGameState, queue: Option<&mut VecDeque<PieceType>>) -> TetrisGameState {
    let mut newState = initialState;

    if initialState.hasHeld{
        return newState;
    }

    newState.hasHeld = true;
    newState.holdPieceType = initialState.currentPieceType;

    if (initialState.holdPieceType == PieceType::None) {
        newState.currentPieceType = initialState.previewQueue[0];

        newState.previewQueue = match queue {
            Some(q) => {
                q.pop_front();
                q.fill_bag();
                q.iter().take(5).cloned().collect::<Vec<_>>().into_iter().chain(repeat(PieceType::None)).take(5).collect::<Vec<_>>()[..5].try_into().expect("Guarenteed to be 5 elements here")
            },
            None => {
                let mut next = [PieceType::default(); 5];
                for (i, p) in initialState.previewQueue[1..5].iter().enumerate() {
                    next[i] = p.clone();
                }
                next
            }
        }
    } else {
        newState.currentPieceType = initialState.holdPieceType;
    }

    newState
}


fn caculateLinesSent(lines_cleared: usize, is_tspin: bool, combo: usize, back_to_back: usize) -> usize {
    let mut linesSent = 0;

    linesSent
}

fn is_full_row(row: [PieceType; 10]) -> bool {
    let mut isFull = true;

    for piece in row {
        if piece == PieceType::None {
            isFull = false;
            break;
        }
    }

    isFull
}