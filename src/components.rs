use std::collections::VecDeque;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

pub struct Player;

pub struct AnimationStep(pub i32);

pub struct Animate(pub bool);

pub struct Speed(pub f32);

pub struct MovementQueue(pub VecDeque<Facing>);

pub struct FacingDirection(pub VecDeque<Facing>);
