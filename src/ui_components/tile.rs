use dioxus::prelude::{*};
use crate::enums::piece_type::{Color, PieceType};

#[derive(PartialEq, Props)]
pub struct TileProps {
    pub piece: PieceType,
}

pub fn Tile(cx: Scope<TileProps>) -> Element {
    let piece = cx.props.piece;

    cx.render(rsx! {
        div {
            background_color: "{piece.get_color()}",
            width: "1em",
            height: "1em"
        }
            
    })
}