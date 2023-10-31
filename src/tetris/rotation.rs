use std::ops::Add;

use crate::enums::rotation::Rotation;


impl Add for Rotation {
    fn add(self, rhs: Self) -> Rotation {
        Rotation::try_from(
            (
                match self {
                    Rotation::None => 0,
                    Rotation::Clock => 1,
                    Rotation::OneEighty => 2,
                    Rotation::Counter => 3,
                } + 
                match rhs {
                    Rotation::None => 0,
                    Rotation::Clock => 1,
                    Rotation::OneEighty => 2,
                    Rotation::Counter => 3,
                }
            ) % 3).expect("Rotation not found for add")
    }

    type Output = Rotation;
}