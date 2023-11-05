
use std::collections::VecDeque;

use dioxus::prelude::{*, GlobalAttributes};

use dioxus_elements::input_data::keyboard_types::Key;

use crate::{ui_components::tetris::Tetris, tetris::{tetris::{TetrisGameState, hard_drop, rotate_90, rotate_180, rotate_270, hold_piece, soft_drop, move_left, move_right}, board::get_piece_starting_pos, queue::Fill}, enums::{piece_type::PieceType, rotation::Rotation}};

pub fn TestTetris(cx: Scope) -> Element {
    let startingBoard = [[PieceType::default(); 10]; 23];


    let pieceQueue = use_state(cx, || {
        let mut queue = VecDeque::<PieceType>::new();
        queue.push_back(PieceType::O);
        queue.push_back(PieceType::O);
        queue.push_back(PieceType::O);
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
            linesSent: 0,
            combo: 0,
            backToBack: 0,
            hasHeld: false,
            piecesPlaced: 0,
    }});

    // use_coroutine(cx, |_: UnboundedReceiver<i32>| {
    //     to_owned![tetrisState];
    //     to_owned![pieceQueue];

    //     let mut currTetrisState = *tetrisState.get();
    //     let mut currTetrisQueue = pieceQueue.get().clone();

    //     async move {
    //         loop {
    //             tokio::time::sleep(Duration::from_millis(1000)).await;
    //             let mut newQueue = currTetrisQueue.clone();
    //             let newState = hard_drop(currTetrisState, Some(&mut newQueue));
                
    //             currTetrisQueue = newQueue.clone();
    //             currTetrisState = newState;
                
    //             tetrisState.set(newState);
    //             pieceQueue.set(newQueue);
                

    //             // println!();
    //             // println!();
    //             // for i in 0..23 {
    //             //     println!();
    //             //     for j in 0..10 {
    //             //         print!("{0}", tetrisState.boardState[i][j]);
    //             //     }
    //             // }

    //         }
    //     }
    // });

    let handleInput = move |event: Event<KeyboardData>| {
       to_owned![tetrisState];
       to_owned![pieceQueue];

        match event.data.key() {
            Key::Character(d) => {
                print!("{0}", d);
                let input = d.to_lowercase();

                match input.as_str() {
                    "f" => {
                        let mut newQueue = pieceQueue.get().clone();
                        let newState = hard_drop(tetrisState.get().clone(), Some(&mut newQueue));
                        
                        tetrisState.set(newState);
                        pieceQueue.set(newQueue);
                        
                    },
                    "e" => {
                        tetrisState.set(rotate_270(tetrisState.get().clone()))
                    },
                    "w" => {
                        tetrisState.set(rotate_180(tetrisState.get().clone()))
                    },
                    "q" => {
                        let mut newQueue = pieceQueue.get().clone();
                        let newState = hold_piece(tetrisState.get().clone(), Some(&mut newQueue));
                        
                        tetrisState.set(newState);
                        pieceQueue.set(newQueue);
                    },
                    "k" => {
                        tetrisState.set(soft_drop(tetrisState.get().clone()))
                    },
                    "j" => {
                        tetrisState.set(move_left(tetrisState.get().clone()))
                    },

                    "l" => {
                        tetrisState.set(move_right(tetrisState.get().clone()))
                    },

                    "i" => {
                        tetrisState.set(rotate_90(tetrisState.get().clone()))
                    },

                    _ => {}
                }

            },
            _ => {},
        }
    };

    cx.render(rsx! {
        div {        
            onkeypress :  handleInput,
            tabindex: "0",
            prevent_default: "onkeypress",
            "style": "outline: none;",
            Tetris {
                state: tetrisState.get().clone()
            }
        }
    })
}