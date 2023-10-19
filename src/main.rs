#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::{prelude::{*, GlobalAttributes}, html::switch};

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    
    cx.render(rsx! {
        div {    
            Board {

            }
        }
    })
}

fn Board(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "grid",
            grid_template_columns: "repeat(10, 1em)",
            grid_template_rows: "repear(10, 1em)",
            for _ in 0..20 {
                for _ in 0..10 {
                    Tile {

                    }
                }
            }
        }
    })
}

enum PieceType {
    T,
    I,
    J,
    L,
    O,
    S,
    Z,
    None
}



fn Tile(cx: Scope) -> Element {
    fn get_color_by_piecetype(t: PieceType) -> String {
        match t {
          PieceType::I => "lightblue".to_string(),
          PieceType::J => "blue".to_string(),
          PieceType::Z => "green".to_string(),
          PieceType::S => "red".to_string(),
          PieceType::L => "orange".to_string(),
          PieceType::T => "purple".to_string(),
          PieceType::O => "yellow".to_string(),
          PieceType::None => "black".to_string()
        } 
    }

    cx.render(rsx! {
        div {
            background_color: "{get_color_by_piecetype(PieceType::I)}",
            width: "1em",
            height: "1em"
        }
    })
}