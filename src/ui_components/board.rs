use dioxus::prelude::*;
use crate::{enums::piece_type::PieceType, tetris::board::{BOARD_HEIGHT, BOARD_WIDTH}};

use super::tile::Tile;
use std::time::Duration;
use rand::Rng;

use crate::tetris::board::Board;

#[derive(PartialEq, Props)]
pub struct BoardProps {
    pub boardState: Board,
}

pub fn Board(cx: Scope<BoardProps>) -> Element {
    let board = cx.props.boardState;

    // use_coroutine(cx, |_: UnboundedReceiver<i32>| {
    //     to_owned![board];

    //     async move {
    //         loop {
                
    //             let mut rng = rand::thread_rng();
    //             let mut newBoard : Board = Default::default();
    //             for i in 0..23 {
    //                 for j in 0..10 {
                        
    //                     newBoard[i][j] = PieceType::try_from(rng.gen_range(0..7)).expect("Out of range");
                        
    //                 }
    //             }
    //             board.set(newBoard);
                
                
    //         }
    //     }
    // });

    cx.render(rsx! {
        div {
            display: "grid",
            grid_template_columns: "repeat(10, 1em)",
            grid_template_rows: "repear(10, 1em)",
           
            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    Tile {
                        piece: board[y][x],
                        isTransparent: y < 3 && board[y][x] == PieceType::None
                    }
                }
            }
        }
    })
}
