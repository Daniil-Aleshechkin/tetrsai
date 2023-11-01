use std::collections::VecDeque;

use dioxus::prelude::*;

use crate::enums::piece_type::PieceType;
use crate::enums::rotation::Rotation;
use crate::tetris::tetris::TetrisGameState;
use crate::ui_components::queue::QUEUE_SIZE;
use crate::tetris::board::{Board, Position};

use super::board::Board;
use super::piece::Piece;
use super::queue::Queue;



#[derive(Props, PartialEq)]
pub struct TetrisProps{
    pub state: TetrisGameState
}

pub fn Tetris(cx: Scope<TetrisProps>) -> Element {
    let tetrisGameState = cx.props.state;

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
                    boardState: tetrisGameState.boardState
                }
            }
            div {
                width: "4em",
                padding: "0.5em",
                Queue {
                    queue: tetrisGameState.previewQueue
                }
            }
        }
    })
}