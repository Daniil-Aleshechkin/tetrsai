use num_enum::TryFromPrimitive;

#[derive(Clone, Hash, Debug, PartialEq, Eq, Copy, TryFromPrimitive)]
#[repr(i32)]
pub enum Rotation {
    None, Clock, OneEighty, Counter
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::None
    }
}