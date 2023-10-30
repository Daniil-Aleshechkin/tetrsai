use dioxus::prelude::*;

use crate::enums::piece_type::PieceType;
use crate::enums::rotation::Rotation;

use super::board::Board;
use super::piece::Piece;

#[derive(Clone, Copy, PartialEq)]
struct TetrisGameState {
    currentPieceType: PieceType,
    currentPieceRotation: Rotation,
    currentPieceLocation: (i32, i32),
    holdPieceType: PieceType,
    boardState: Board,
}

pub fn Tetris(cx: Scope) -> Element {
    let startingBoard: Board = [
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I],
        [PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I, PieceType::I]
    ];

    let tetrisGameState = use_state(cx, || TetrisGameState {
        currentPieceType : PieceType::I,
        currentPieceLocation: (3,3),
        currentPieceRotation: Rotation::None,
        boardState: startingBoard,
        holdPieceType: PieceType::L,
    });

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
                    top: "{tetrisGameState.currentPieceLocation.0}em",        
                    left: "{tetrisGameState.currentPieceLocation.1}em",        
                    Piece {
                        piece: tetrisGameState.currentPieceType,
                        rotation: tetrisGameState.currentPieceRotation,
                    },
                }
                Board {
                    startingBoardState: tetrisGameState.boardState
                }
            }
        }
    })
}