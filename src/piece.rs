use dioxus::prelude::{*};

use crate::tile::PieceType;

#[derive(PartialEq, Props)]
struct PieceProps {
    piece: PieceType,
}

fn Piece(cx: Scope<PieceProps>) -> Element {
    //fn get_coords
    
    rsx! {
      div {
        
      }  
    }
}