use std::{collections::VecDeque, iter::repeat};

use crate::{tetris::{tetris::{TetrisGameState, hard_drop, move_left, soft_drop}, board::{Board, Position, get_piece_starting_pos}, queue::Fill}, enums::{piece_type::PieceType, rotation::Rotation}, ui_components::piece};



#[test]
fn test_hard_drop() {
    let startingBoard = [[PieceType::default(); 10]; 23];
    let mut pieceQueue = VecDeque::<PieceType>::new();

    pieceQueue.fill_bag();
    let piece = pieceQueue.pop_back().expect("Starting queue was empty");
    
    let nextPieces = pieceQueue.iter().cloned().take(5).collect::<Vec<_>>()[..5].try_into().expect("Queue wasn't filled for 5 piece queue");

    let gameState = TetrisGameState {
        boardState: startingBoard,
        currentPieceType: piece,
        currentPieceRotation: Rotation::None,
        currentPieceLocation: get_piece_starting_pos(piece),
        holdPieceType: PieceType::None,
        previewQueue: nextPieces,
        isLose: false,
        linesSent: 0,
            combo: 0,
            backToBack: 0,
            hasHeld: false
    };

    let newGameState = hard_drop(gameState, Some(&mut pieceQueue)); 
    let newNewGameState = hard_drop(newGameState, Some(&mut pieceQueue));
}


#[test]
fn test_full_hard_drop() {
    let startingBoard = [[PieceType::default(); 10]; 23];
    let mut pieceQueue = VecDeque::<PieceType>::new();

    pieceQueue.push_back(PieceType::O);
    pieceQueue.push_back(PieceType::O);
    pieceQueue.push_back(PieceType::O);
    let piece = pieceQueue.pop_back().expect("Starting queue was empty");
    
    let nextPieces = pieceQueue.iter().cloned().chain(repeat(PieceType::None)).take(5).collect::<Vec<_>>()[..5].try_into().expect("Queue wasn't filled for 5 piece queue");

    let gameState = TetrisGameState {
        boardState: startingBoard,
        currentPieceType: piece,
        currentPieceRotation: Rotation::None,
        currentPieceLocation: get_piece_starting_pos(piece),
        holdPieceType: PieceType::None,
        previewQueue: nextPieces,
        isLose: false,
        linesSent: 0,
        combo: 0,
        backToBack: 0,
        hasHeld: false
    };

    let mut newGameState =  hard_drop(gameState, Some(&mut pieceQueue));
    newGameState = soft_drop(newGameState);
}
