#![allow(non_snake_case)]
use dioxus::prelude::{*};

mod board;
mod enums;
mod tile;

use board::Board;
use enums::piece_type::PieceType;

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