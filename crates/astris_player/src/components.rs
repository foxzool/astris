use crate::PlayerAction;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/// 标记：玩家控制的实体。
#[derive(Component)]
pub struct PlayerControlled;

impl PlayerControlled {
    pub fn default_input_map() -> InputMap<PlayerAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use super::PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(Up, KeyCode::KeyW);
        input_map.insert(Up, GamepadButton::DPadUp);

        input_map.insert(Down, KeyCode::KeyS);
        input_map.insert(Down, GamepadButton::DPadDown);

        input_map.insert(Left, KeyCode::KeyA);
        input_map.insert(Left, GamepadButton::DPadLeft);

        input_map.insert(Right, KeyCode::KeyD);
        input_map.insert(Right, GamepadButton::DPadRight);

        // Abilities
        input_map.insert(Ability1, KeyCode::Digit1);
        input_map.insert(Ability1, GamepadButton::West);
        input_map.insert(Ability1, MouseButton::Left);

        input_map.insert(Ability2, KeyCode::Digit2);
        input_map.insert(Ability2, GamepadButton::North);
        input_map.insert(Ability2, MouseButton::Right);

        input_map.insert(Ability3, KeyCode::Digit3);
        input_map.insert(Ability3, GamepadButton::East);

        input_map.insert(Ability4, KeyCode::Space);
        input_map.insert(Ability4, GamepadButton::South);

        input_map.insert(Ultimate, KeyCode::Digit4);
        input_map.insert(Ultimate, GamepadButton::LeftTrigger2);

        input_map
    }
}
