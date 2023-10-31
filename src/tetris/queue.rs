use std::collections::VecDeque;

use crate::enums::piece_type::PieceType;

use rand::Rng;

fn generate_bag() -> [PieceType; 7] {
    let mut bag = [PieceType::I, PieceType::J, PieceType::L, PieceType::O, PieceType::S, PieceType::T, PieceType::Z];
    let mut rng = rand::thread_rng();

    for i in 0..7 {
        bag.swap(i, rng.gen_range(0..7));
    }

    *bag
}

trait Fill {
    pub fn fill_bag(&self) {}
}

impl Fill for VecDeque<PieceType> {
    fn fill_bag(&self) {
        if self.len() <= 7 {
            self.extend(generate_bag());
            self.extend(generate_bag());
        } else if self.len() <= 14 {
            self.extend(generate_bag());
        }
    }
}