use nalgebra::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pawn {
    pub currentState: PawnState,
    pub nextState: PawnState,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PawnState {
    Idle,
    Moving(Vector2<f32>),
    Attacking(u32),
    AttackMoving(Vector2<f32>),
    Stopped,
    Casting(u32),     //Should these be ability data objects?
    Channelling(u32), //Should these be ability dat objects?
    Dead,
}
