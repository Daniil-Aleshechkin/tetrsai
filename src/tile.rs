use dioxus::prelude::{*};
use crate::enums::piece_type::PieceType;

#[derive(PartialEq, Props)]
pub struct TileProps {
    pub piece: PieceType,
}

pub fn Tile(cx: Scope<TileProps>) -> Element {
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

    let piece = cx.props.piece;

    cx.render(rsx! {
        div {
            background_color: "{get_color_by_piecetype(piece)}",
            width: "1em",
            height: "1em"
        }
    })
}