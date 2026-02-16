use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum MovementType {
    Line,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EnemyType {
    Normal,
    Large,
    Boss,
}