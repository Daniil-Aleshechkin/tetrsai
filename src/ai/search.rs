use std::collections::{HashSet, VecDeque};
use strum::IntoEnumIterator;

use crate::{tetris::{tetris::{TetrisGameState, get_next_state}, board::{Position, Board}}, enums::{piece_type::PieceType, rotation::Rotation, action::Action}};

#[derive(Clone, Copy, Hash)]
struct PieceState {
    pub currentPieceType: PieceType,
    pub currentPieceRotation: Rotation,
    pub currentPieceLocation: Position,
    pub holdPieceType: PieceType,
    pub board: Board,
}

impl PieceState {
    fn from_game_state(gameState: TetrisGameState) -> PieceState {
        PieceState { currentPieceType: gameState.currentPieceType, currentPieceRotation: gameState.currentPieceRotation, currentPieceLocation: gameState.currentPieceLocation, holdPieceType: gameState.holdPieceType, board: gameState.boardState }
    }
}

impl PartialEq for PieceState {
    fn eq(&self, other: &Self) -> bool {
        if self.currentPieceType == PieceType::O && other.currentPieceType == PieceType::O {
            self.currentPieceLocation == other.currentPieceLocation
                && self.holdPieceType == other.holdPieceType
                && self.board == other.board
        } else {
            self.currentPieceType == other.currentPieceType
                && self.currentPieceRotation == other.currentPieceRotation
                && self.currentPieceLocation == other.currentPieceLocation
                && self.holdPieceType == other.holdPieceType
                && self.board == other.board
        }
    }
}

impl Eq for PieceState {}

fn push_actions_from(gameState: TetrisGameState, visited: &mut HashSet<PieceState>, finalState: &mut Vec<TetrisGameState>) -> Vec<TetrisGameState> {
    let mut nextStates = Vec::<TetrisGameState>::new();
    
    for action in Action::iter() {
        let nextState = get_next_state(action, gameState, None);

        let pieceState = PieceState::from_game_state(nextState);

        if visited.contains(&pieceState) {
            continue;
        }

        visited.insert(pieceState);

        if action == Action::HardDrop {
            finalState.push(nextState);
        } else {
            nextStates.push(nextState);
        }
    }

    nextStates
}

pub fn get_all_next_states(gameState: TetrisGameState) -> Vec<TetrisGameState> {
    let mut visited = HashSet::<PieceState>::new();

    visited.insert(PieceState::from_game_state(gameState));

    let mut fring = VecDeque::<TetrisGameState>::new();

    fring.push_back(gameState);

    let mut finalStates = Vec::<TetrisGameState>::new();
    
    while !fring.is_empty() {
        match fring.pop_front() {
            Some(nextState) => 
                {
                    let nextStates = push_actions_from(nextState, &mut visited, &mut finalStates);
                    
                    for state in nextStates {
                        fring.push_back(state);
                    }
                 },
            None => break,
        }
    }

    finalStates
}