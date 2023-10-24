#![allow(non_snake_case)]
use dioxus::prelude::{*};

// Import the other modules
mod board;
mod tile;

use board::Board;
use tile::PieceType;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
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
    
    cx.render(rsx! {
        div {    
            Board {
                startingBoardState: startingBoard
            }
        }
    })
}