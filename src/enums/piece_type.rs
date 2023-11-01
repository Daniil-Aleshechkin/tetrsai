use std::fmt::{Display, self};

use num_enum::TryFromPrimitive;

use crate::tetris::board::Position;

use super::rotation::Rotation;

#[derive(Clone, Debug, PartialEq, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum PieceType {
    T, I, J, L, O, S, Z, None
}


impl Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            PieceType::T => "T",
            PieceType::I => "I",
            PieceType::J => "J",
            PieceType::L => "L",
            PieceType::O => "O",
            PieceType::S => "S",
            PieceType::Z => "Z",
            PieceType::None => "N",
        };
        write!(f, "{}", printable)
    }
}

impl Default for PieceType {
    fn default() -> Self {
        PieceType::None
    }
}

pub trait PositionMap {
    fn get_position_map(&self, rotation: Rotation) -> Option<[Position; 4]>;
}

pub trait Color {
    fn get_color(&self) -> String;
}

impl Color for PieceType {
    fn get_color(&self) -> String {
        match self {
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
}

impl PieceType {
    fn t_positions(&self, rotation: Rotation) -> [(usize, usize); 4] {
        match rotation {
            Rotation::None => [(0, 1), (1, 0), (1, 1), (2, 1)],
            Rotation::Clock => [(1, 0), (1, 1), (2, 1), (1, 2)],
            Rotation::OneEighty => [(0, 1), (1, 1), (2, 1), (1, 2)],
            Rotation::Counter => [(1, 0), (0, 1), (1, 1), (1, 2)],
        }
    }

    fn i_positions(&self, rotation: Rotation) -> [(usize, usize); 4] {
        match rotation {
            Rotation::None => [(0, 1), (1, 1), (2, 1), (3, 1)],
            Rotation::Clock => [(2, 0), (2, 1), (2, 2), (2, 3)],
            Rotation::OneEighty => [(0, 2), (1, 2), (2, 2), (3, 2)],
            Rotation::Counter => [(1, 0), (1, 1), (1, 2), (1, 3)],
        }
    }

    fn j_positions(&self, rotation: Rotation) -> [(usize, usize); 4] {
        match rotation {
            Rotation::None => [(0, 0), (0, 1), (1, 1), (2, 1)],
            Rotation::Clock => [(1, 0), (2, 0), (1, 1), (1, 2)],
            Rotation::OneEighty => [(0, 1), (1, 1), (2, 1), (2, 2)],
            Rotation::Counter => [(0, 2), (1, 2), (1, 1), (1, 0)],
        }
    }

    fn l_positions(&self, rotation: Rotation) -> [(usize, usize); 4] {
        match rotation {
            Rotation::None => [(2, 0), (0, 1), (1, 1), (2, 1)],
            Rotation::Clock => [(2, 2), (1, 2), (1, 1), (1, 0)],
            Rotation::OneEighty => [(0, 1), (1, 1), (2, 1), (0, 2)],
            Rotation::Counter => [(1, 0), (0, 0), (1, 1), (1, 2)],
        }
    }

    fn s_positions(&self, rotation: Rotation) -> [(usize, usize); 4] {
        match rotation {
            Rotation::None => [(1, 0), (2, 0), (0, 1), (1, 1)],
            Rotation::Clock => [(1, 0), (1, 1), (2, 1), (2, 2)],
            Rotation::OneEighty => [(0, 2), (1, 2), (1, 1), (2, 1)],
            Rotation::Counter => [(0, 0), (0, 1), (1, 1), (1, 2)],
        }
    }

    fn z_positions(&self, rotation: Rotation) -> [(usize, usize); 4] {
        match rotation {
            Rotation::None => [(0, 0), (2, 1), (1, 0), (1, 1)],
            Rotation::Clock => [(2, 0), (1, 1), (2, 1), (1, 2)],
            Rotation::OneEighty => [(0, 1), (1, 2), (1, 1), (2, 2)],
            Rotation::Counter => [(0, 1), (1, 0), (1, 1), (0, 2)],
        }
    }
    fn o_positions(&self, _: Rotation) -> [(usize, usize); 4] {
        [(0,0), (0,1), (1,0), (1,1)]
    }
}

impl PositionMap for PieceType {
    fn get_position_map(&self, rotation: Rotation) -> Option<[Position; 4]> {
        match *self {
            PieceType::T => Some(self.t_positions(rotation).map(|x| Position::from_tuple(x))),
            PieceType::I => Some(self.i_positions(rotation).map(|x| Position::from_tuple(x))),
            PieceType::J => Some(self.j_positions(rotation).map(|x| Position::from_tuple(x))),
            PieceType::L => Some(self.l_positions(rotation).map(|x| Position::from_tuple(x))),
            PieceType::O => Some(self.o_positions(rotation).map(|x| Position::from_tuple(x))),
            PieceType::S => Some(self.s_positions(rotation).map(|x| Position::from_tuple(x))),
            PieceType::Z => Some(self.z_positions(rotation).map(|x| Position::from_tuple(x))),
            PieceType::None => None,
        }
    }
}