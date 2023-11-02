use std::iter::repeat;
use std::collections::VecDeque;

use crate::enums::piece_type::PositionMap;
use crate::enums::{piece_type::PieceType, rotation::Rotation};
use super::queue::Fill;
use super::board::{Board, Position, get_piece_starting_pos, get_lowest_piece_board_pos, BOARD_HEIGHT};

#[derive(Copy, Clone, PartialEq)]
pub struct TetrisGameState {
    pub currentPieceType: PieceType,
    pub currentPieceRotation: Rotation,
    pub currentPieceLocation: Position,
    pub holdPieceType: PieceType,
    pub boardState: Board,
    pub previewQueue: [PieceType; 5],
    pub isLose: bool,
    pub linesSent: i32,
    pub combo: i32,
    pub backToBack: i32,
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
        
        if initialState.boardState[yPos][xPos] != PieceType::None {
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

        newState.boardState[yPos][xPos] = initialState.currentPieceType;
    }
    
    let nextBoardNonClearedLines = newState.boardState.iter().cloned().filter(|&row| !is_full_row(row)).collect::<Vec<_>>();

    let linesCleared = BOARD_HEIGHT - nextBoardNonClearedLines.len(); 

    newState.boardState = repeat([PieceType::default(); 10]).take(linesCleared).chain(nextBoardNonClearedLines.into_iter()).collect::<Vec<_>>().try_into().expect("linescleared + filtered lines should board height");

    newState.isLose = isLoseFromCurrentPiece || isLoseFromPlacement;

    newState
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