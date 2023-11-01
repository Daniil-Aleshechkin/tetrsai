use std::{collections::VecDeque, time::Duration};

use dioxus::prelude::*;

use crate::{ui_components::tetris::Tetris, tetris::{tetris::{TetrisGameState, hard_drop}, queue::Fill, board::get_piece_starting_pos}, enums::{piece_type::PieceType, rotation::Rotation}};

pub fn TestTetris(cx: Scope) -> Element {
    let startingBoard = [[PieceType::default(); 10]; 23];


    let pieceQueue = use_state(cx, || {
        let mut queue = VecDeque::<PieceType>::new();
        queue.fill_bag();
        queue
    });
    
    let tetrisState = use_state(cx, || {
        let pieceQueue = pieceQueue.to_owned();
        let mut newPieceQueue = pieceQueue.get().clone();

        let piece = newPieceQueue.pop_front().expect("Starting queue was empty");

        let nextPieces = newPieceQueue.iter().cloned().take(5).collect::<Vec<_>>()[..5].try_into().expect("Queue wasn't filled for 5 piece queue");

        pieceQueue.set(newPieceQueue);

        TetrisGameState {
            boardState: startingBoard,
            currentPieceType: piece,
            currentPieceRotation: Rotation::None,
            currentPieceLocation: get_piece_starting_pos(piece),
            holdPieceType: PieceType::None,
            previewQueue: nextPieces,
            isLose: false,
    }});

    use_coroutine(cx, |_: UnboundedReceiver<i32>| {
        to_owned![tetrisState];
        to_owned![pieceQueue];

        let mut currTetrisState = *tetrisState.get();
        let mut currTetrisQueue = pieceQueue.get().clone();

        async move {
            loop {
                tokio::time::sleep(Duration::from_millis(1000)).await;
                let mut newQueue = currTetrisQueue.clone();
                let newState = hard_drop(currTetrisState, Some(&mut newQueue));
                
                currTetrisQueue = newQueue.clone();
                currTetrisState = newState;
                
                tetrisState.set(newState);
                pieceQueue.set(newQueue);
                

                // println!();
                // println!();
                // for i in 0..23 {
                //     println!();
                //     for j in 0..10 {
                //         print!("{0}", tetrisState.boardState[i][j]);
                //     }
                // }

            }
        }
    });

    cx.render(rsx! {
        Tetris {
            state: tetrisState.get().clone()
        }
    })
}