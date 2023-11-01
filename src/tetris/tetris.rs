use std::iter::repeat;
use std::{collections::VecDeque, cell::RefCell, rc::Rc};

use crate::enums::piece_type::PositionMap;
use crate::enums::{piece_type::PieceType, rotation::Rotation};
use super::queue::Fill;
use super::board::{Board, Position, get_piece_starting_pos, get_lowest_piece_board_pos};

#[derive(Copy, Clone, PartialEq)]
pub struct TetrisGameState {
    pub currentPieceType: PieceType,
    pub currentPieceRotation: Rotation,
    pub currentPieceLocation: Position,
    pub holdPieceType: PieceType,
    pub boardState: Board,
    pub previewQueue: [PieceType; 5],
    pub isLose: bool
}

pub fn hard_drop(initialState: TetrisGameState, queue: Option<&mut VecDeque<PieceType>>) -> TetrisGameState {

    if initialState.currentPieceType == PieceType::None || initialState.isLose {
        return initialState;
    }

    let mut newState: TetrisGameState = initialState;

    let nextPiece = initialState.previewQueue[0];

    let next = match queue {
        Some(q) => {
            q.pop_front();
            q.fill_bag();
            q.iter().take(5).cloned().collect::<Vec<_>>()
        },
        None => {
            initialState.previewQueue[1..5].to_vec()
        },
    }.into_iter().chain(repeat(PieceType::None)).take(5).collect::<Vec<_>>();

    newState.previewQueue = next[..5].try_into().expect("Guarenteed to be 5 elements here");

    newState.currentPieceLocation = get_piece_starting_pos(nextPiece);
    newState.currentPieceRotation = Rotation::None;
    newState.currentPieceType = nextPiece;

    let mut isLoseFromCurrentPiece = false;
    for offset in newState.currentPieceType.get_position_map(newState.currentPieceRotation).expect("Guard exists for None") {
        let xPos = offset.x + newState.currentPieceLocation.x;
        let yPos = offset.y + newState.currentPieceLocation.y;
        
        if initialState.boardState[yPos][xPos] != PieceType::None {
            print!("CURRENT PIECE RENDER LOSS");
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
    
    newState.isLose = isLoseFromCurrentPiece || isLoseFromPlacement;

    newState
}