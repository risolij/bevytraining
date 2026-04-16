use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub enum Facing {
    Up,
    Left,
    #[default]
    Down,
    Right
}

impl Facing {
    pub fn from_velocity(velocity: Vec2) -> Self {
        if velocity.x.abs() > velocity.y.abs() {
            if velocity.x > 0.0 { Facing::Right } else { Facing::Left }
        } else {
            if velocity.y > 0.0 { Facing::Up } else { Facing::Down }
        }
    }

    pub(crate) fn direction_index(self) -> usize {
        match self {
            Self::Up => 0,
            Self::Left  => 1,
            Self::Down  => 2,
            Self::Right => 3,
        }
    }
}
