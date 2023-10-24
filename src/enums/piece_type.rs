use num_enum::TryFromPrimitive;

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
