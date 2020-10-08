use crate::components::*;

pub fn facing_to_row(facing: &Facing) -> i32 {
    match facing {
        Facing::South => 0,
        Facing::West => 1,
        Facing::East => 2,
        Facing::North => 3,
    }
}