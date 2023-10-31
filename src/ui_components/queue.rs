use dioxus::prelude::*;

use crate::enums::{piece_type::PieceType, rotation::Rotation};
use super::piece::Piece;

#[derive(PartialEq, Props)]
pub struct QueueProps {
    queue: [PieceType; QUEUE_SIZE],
}

pub const QUEUE_SIZE : usize = 5;

pub fn Queue(cx: Scope<QueueProps>) -> Element {

    let renderedQueue = cx.props.queue;
    
    cx.render(rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            for piece in renderedQueue {
                div {
                    height: "3em",        
                    Piece {
                        piece: piece,
                        rotation: Rotation::None,
                    }
                }
            }
        }
    ))
}
