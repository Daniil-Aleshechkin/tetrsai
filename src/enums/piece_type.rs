use num_enum::TryFromPrimitive;

use super::rotation::Rotation;

#[derive(Clone, Debug, PartialEq, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum PieceType {
    T, I, J, L, O, S, Z, None
}

impl Default for PieceType {
    fn default() -> Self {
        PieceType::None
    }
}

pub trait PositionMap {
    fn get_position_map(&self, rotation: Rotation) -> Option<[(i32, i32); 4]>;
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
    fn t_positions(&self, rotation: Rotation) -> [(i32, i32); 4] {
        match rotation {
            Rotation::None => [(0, 1), (1, 0), (1, 1), (2, 1)],
            Rotation::Clock => [(1, 0), (1, 1), (2, 1), (1, 2)],
            Rotation::OneEighty => [(0, 1), (1, 1), (2, 1), (1, 2)],
            Rotation::Counter => [(1, 0), (0, 1), (1, 1), (1, 2)],
        }
    }

    fn i_positions(&self, rotation: Rotation) -> [(i32, i32); 4] {
        match rotation {
            Rotation::None => [(0, 1), (1, 1), (2, 1), (3, 1)],
            Rotation::Clock => [(2, 0), (2, 1), (2, 2), (2, 3)],
            Rotation::OneEighty => [(0, 2), (1, 2), (2, 2), (3, 2)],
            Rotation::Counter => [(1, 0), (1, 1), (1, 2), (1, 3)],
        }
    }

    fn j_positions(&self, rotation: Rotation) -> [(i32, i32); 4] {
        match rotation {
            Rotation::None => [(0, 0), (0, 1), (1, 1), (2, 1)],
            Rotation::Clock => [(1, 0), (2, 0), (1, 1), (1, 2)],
            Rotation::OneEighty => [(0, 1), (1, 1), (2, 1), (2, 2)],
            Rotation::Counter => [(0, 2), (1, 2), (1, 1), (1, 0)],
        }
    }

    fn l_positions(&self, rotation: Rotation) -> [(i32, i32); 4] {
        match rotation {
            Rotation::None => [(2, 0), (0, 1), (1, 1), (2, 1)],
            Rotation::Clock => [(2, 2), (1, 2), (1, 1), (1, 0)],
            Rotation::OneEighty => [(0, 1), (1, 1), (2, 1), (0, 2)],
            Rotation::Counter => [(1, 0), (0, 0), (1, 1), (1, 2)],
        }
    }

    fn s_positions(&self, rotation: Rotation) -> [(i32, i32); 4] {
        match rotation {
            Rotation::None => [(1, 0), (2, 0), (0, 1), (1, 1)],
            Rotation::Clock => [(1, 0), (1, 1), (2, 1), (2, 2)],
            Rotation::OneEighty => [(0, 2), (1, 2), (1, 1), (2, 1)],
            Rotation::Counter => [(0, 0), (0, 1), (1, 1), (1, 2)],
        }
    }

    fn z_positions(&self, rotation: Rotation) -> [(i32, i32); 4] {
        match rotation {
            Rotation::None => [(0, 0), (2, 1), (1, 0), (1, 1)],
            Rotation::Clock => [(2, 0), (1, 1), (2, 1), (1, 2)],
            Rotation::OneEighty => [(0, 1), (1, 2), (1, 1), (2, 2)],
            Rotation::Counter => [(0, 1), (1, 0), (1, 1), (0, 2)],
        }
    }
    fn o_positions(&self, _: Rotation) -> [(i32, i32); 4] {
        [(0,0), (0,1), (1,0), (1,1)]
    }
}

impl PositionMap for PieceType {
    fn get_position_map(&self, rotation: Rotation) -> Option<[(i32, i32); 4]> {
        match *self {
            PieceType::T => Some(self.t_positions(rotation)),
            PieceType::I => Some(self.i_positions(rotation)),
            PieceType::J => Some(self.j_positions(rotation)),
            PieceType::L => Some(self.l_positions(rotation)),
            PieceType::O => Some(self.o_positions(rotation)),
            PieceType::S => Some(self.s_positions(rotation)),
            PieceType::Z => Some(self.z_positions(rotation)),
            PieceType::None => None,
        }
    }
}