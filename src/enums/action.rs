use strum_macros::EnumIter;

#[derive(Clone, Debug, PartialEq, EnumIter, Copy)]
#[repr(i32)]
pub enum Action {
    SoftDrop, HardDrop, MoveLeft, MoveRight, HoldPiece, Rotate90, Rotate180, Rotate270 
}