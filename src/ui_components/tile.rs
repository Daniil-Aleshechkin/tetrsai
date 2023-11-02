use dioxus::prelude::{*};
use crate::enums::piece_type::{Color, PieceType};

#[derive(PartialEq, Props)]
pub struct TileProps {
    pub piece: PieceType,
    pub isTransparent: bool,
}

pub fn Tile(cx: Scope<TileProps>) -> Element {
    let piece = cx.props.piece;
    let isTransparent = cx.props.isTransparent;

    let backgroundColor = if isTransparent {  "white".to_string() } else { piece.get_color()};
    cx.render(rsx! {
        div {
            background_color: "{backgroundColor}",
            enable_background: "{isTransparent}",
            width: "1em",
            height: "1em"
        }
            
    })
}