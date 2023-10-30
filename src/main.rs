#![allow(non_snake_case)]
use std::time::Duration;

use dioxus::prelude::{*};

mod ui_components;
mod enums;

use enums::piece_type::PieceType;
use enums::rotation::Rotation;

use rand::Rng;

use crate::ui_components::tetris::Tetris;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    
    let piece = use_state(cx, || { PieceType::I});
    let rotation = use_state(cx, || { Rotation::None});

    use_coroutine(cx, |_: UnboundedReceiver<i32>| {
        to_owned![piece];
        to_owned![rotation];

        async move {
            loop {
                let mut rng = rand::thread_rng();
                
                piece.set( PieceType::try_from(rng.gen_range(0..7)).expect("No mapping"));
                //rotation.set(Rotation::try_from(rng.gen_range(0..4)).expect("No mapping"));
                rotation.set(Rotation::Clock);
                tokio::time::sleep(Duration::from_millis(1000)).await;
                
            }
        }
    });

    cx.render(rsx! {
        Tetris {
            
        }
    })
}