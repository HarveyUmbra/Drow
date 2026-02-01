use bevy::prelude::*;

#[derive(Component)]
pub struct Selected;

/// The component marks the entity that controls the player.
#[derive(Component)]
pub struct Player;

/// The component marks the entity use Navigation
#[derive(Component, Default)]
pub struct Navigator;
