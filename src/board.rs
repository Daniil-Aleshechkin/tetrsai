use dioxus::{prelude::*, html::switch};
use crate::enums::piece_type::PieceType;
use crate::tile::Tile;

pub type Board = [[PieceType; 10]; 23];

#[derive(PartialEq, Props)]
pub struct BoardProps {
    pub startingBoardState: Board,
}

pub fn Board(cx: Scope<BoardProps>) -> Element {
    let mut board = cx.props.startingBoardState.clone();

    cx.render(rsx! {
        div {
            display: "grid",
            grid_template_columns: "repeat(10, 1em)",
            grid_template_rows: "repear(10, 1em)",
            for i in 0..23 {
                for j in 0..10 {
                    Tile {
                        piece: board[i][j]
                    }
                }
            }
        }
    })
}
