#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

use crate::{enums::{piece_type::PieceType, rotation::Rotation}, tetris::{tetris::TetrisGameState, board::Position}, ai::search::get_all_next_states};

#[test]
fn test_get_all_next_actions() {
    let board = [[PieceType::default(); 10]; 23];
    let queue = [PieceType::O, PieceType::O, PieceType::O, PieceType::O, PieceType::O];

    let tetrisGameState = TetrisGameState {
        currentPieceType: PieceType::O,
        currentPieceRotation: Rotation::None,
        currentPieceLocation: Position::from_tuple((3,0)),
        holdPieceType: PieceType::None,
        boardState: board,
        previewQueue: queue,
        isLose: false,
        linesSent: 0,
        combo: 0,
        backToBack: 0,
        hasHeld: false,
        piecesPlaced: 0,
    };

    let nextStates = get_all_next_states(tetrisGameState);
}