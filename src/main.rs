#![allow(non_snake_case)]
use dioxus::prelude::{*};

mod ui_components;
mod enums;
mod tetris;

#[cfg(test)]
mod tests;

use crate::ui_components::test_tetris::TestTetris;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        TestTetris {
            
        }
    })
}