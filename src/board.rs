use dioxus::prelude::{*};
use crate::tile::{Tile, PieceType}; // Importing the Tile component and PieceType from tile module
use rand::Rng;
use core::time::Duration;

pub type Board = [[PieceType; 10]; 23];

#[derive(PartialEq, Props)]
pub struct BoardProps {
    pub startingBoardState: Board,
}

pub fn Board(cx: Scope<BoardProps>) -> Element {
    let board = use_state(cx, || cx.props.startingBoardState.clone());

    use_coroutine(cx, |_: UnboundedReceiver<i32>| {
        to_owned![board];

        async move {
            loop {
                let mut rng = rand::thread_rng();
                let mut newBoard : Board = Default::default();
                for i in 0..23 {
                    for j in 0..10 {
                        
                        newBoard[i][j] = PieceType::try_from(rng.gen_range(0..7)).expect("Out of range");
                        
                    }
                }
                board.set(newBoard);
                tokio::time::sleep(Duration::from_millis(10)).await;
                
            }
        }
    });

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