use std::collections::{VecDeque, BinaryHeap};

use crate::{tetris::tetris::TetrisGameState, enums::piece_type::PieceType};

use super::search::get_all_next_states;

#[derive(PartialEq, Eq)]
struct ExpandedState {
    states: Vec<TetrisGameState>,
    cost: f32,
}

impl Ord for ExpandedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for ExpandedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_ucs_cost(gameState: TetrisGameState) -> f32 {
    board_height(gameState)
}

fn get_ucs_next_move(gameState: TetrisGameState ) -> Option<TetrisGameState> {
    let mut fring = BinaryHeap::<ExpandedState>::new();

    fring.append(ExpandedState{
        states: Vec::from([gameState]),
        cost: 0,
    });

    while !fring.is_empty() {
        match fring.pop() {
            Some(state) => {
                if state.states.last().expect("Should not be deleting").previewQueue = [PieceType::default(); 5] {
                    return state.states.get(1);
                }
                
                let newStates = get_all_next_states(state.states.last().expect("No deleting should happen"));

                for newState in newStates {
                    let mut oldSequence = state.states.clone();
                    oldSequence.push(newState);
                    fring.append(ExpandedState{
                        states: oldSequence,
                        cost: state.cost() + get_ucs_cost(newState) 
                    });
                }
            },
            None => break,
        }
    }

    None
}