use std::collections::VecDeque;

use dioxus::prelude::*;

use crate::enums::piece_type::PieceType;
use crate::enums::rotation::Rotation;
use crate::ui_components::queue::QUEUE_SIZE;
use crate::tetris::board::{Board, Position};

use super::board::Board;
use super::piece::Piece;
use super::queue::Queue;


#[derive(Clone, PartialEq)]
struct TetrisGameState {
    currentPieceType: PieceType,
    currentPieceRotation: Rotation,
    currentPieceLocation: Position,
    holdPieceType: PieceType,
    boardState: Board,
    queue: VecDeque<PieceType>,
}

pub fn Tetris(cx: Scope) -> Element {
    // let startingBoard: Board = [
    //     [PieceType::None, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
    //     [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I]
    // ];

    let startingBoard = [[PieceType::default(); 10]; 23];

    let tetrisGameState = use_state(cx, || TetrisGameState {
        currentPieceType : PieceType::I,
        currentPieceLocation: Position::from_tuple((3,3)),
        currentPieceRotation: Rotation::None,
        boardState: startingBoard,
        holdPieceType: PieceType::L,
        queue: VecDeque::from(vec![PieceType::L, PieceType::I, PieceType::T, PieceType::O, PieceType::J, PieceType::T ])
    });

    fn get_viewable_piece_queue(queue: &VecDeque<PieceType>) -> [PieceType; 5] {
        let mut viewableQueue = [PieceType::default(); QUEUE_SIZE];

        for (i, piece) in queue.iter().take(QUEUE_SIZE).enumerate() {
            viewableQueue[i] = *piece;
        }

        viewableQueue
    }

    cx.render(rsx! {
        div {
            display: "flex",
            div {
                width: "4em",
                height: "4em",        
                padding: "0.5em",
                Piece {
                    piece: tetrisGameState.holdPieceType,
                    rotation: Rotation::None,
                },
            }
            div {    
                position: "relative",
                div {
                    position: "absolute",
                    top: "{tetrisGameState.currentPieceLocation.y}em",        
                    left: "{tetrisGameState.currentPieceLocation.x}em",        
                    Piece {
                        piece: tetrisGameState.currentPieceType,
                        rotation: tetrisGameState.currentPieceRotation,
                    },
                }
                Board {
                    startingBoardState: tetrisGameState.boardState
                }
            }
            div {
                width: "4em",
                padding: "0.5em",
                Queue {
                    queue: get_viewable_piece_queue(&tetrisGameState.queue)
                }
            }
        }
    })
}