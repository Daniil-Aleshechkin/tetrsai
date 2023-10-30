use dioxus::prelude::*;

use crate::enums::{piece_type::*, rotation::Rotation};

#[derive(PartialEq, Props)]
pub struct PieceProps {
    piece: PieceType,
    rotation: Rotation,
}

pub fn Piece(cx: Scope<PieceProps>) -> Element {
    let piece = cx.props.piece;
    let rotation = cx.props.rotation;

    match piece.get_position_map(rotation) {
        Some(positionMap) => cx.render(rsx! {
            div {
                position: "relative",
                for coord in positionMap {
                    div {
                        position: "absolute",
                        left : "{coord.0}em",
                        top : "{coord.1}em",
                        width : "1em",
                        height : "1em",
                        background_color: "{piece.get_color()}",
                    }
                }
            }
        }),
        None => cx.render(rsx! {
            div {
                
            }
        })
    }
}