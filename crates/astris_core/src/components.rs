use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Faction(pub FactionKind);

pub enum FactionKind {
    Player,
    Npc,
    Monster,
}
