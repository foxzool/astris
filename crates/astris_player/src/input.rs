use bevy::math::Dir2;
use bevy::prelude::Reflect;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Ability1,
    Ability2,
    Ability3,
    Ability4,
    Ultimate,
}

impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub(crate) const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    pub(crate) fn direction(self) -> Option<Dir2> {
        match self {
            PlayerAction::Up => Some(Dir2::Y),
            PlayerAction::Down => Some(Dir2::NEG_Y),
            PlayerAction::Left => Some(Dir2::NEG_X),
            PlayerAction::Right => Some(Dir2::X),
            _ => None,
        }
    }
}
