//! Spawn the main level.

use bevy::{ecs::world::Command, prelude::*};
use bevy::ecs::system::RunSystemOnce;
// use bevy_ecs_tiled::prelude::{TiledMap, TiledMapBundle, TiledMapPlugin};
// use bevy_ecs_tilemap::prelude::TilemapPlugin;

use crate::demo::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    // app.add_plugins(TilemapPlugin)
    //     .add_plugins(TiledMapPlugin);
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

struct SpawnLevel {
    // map_handle: Handle<TiledMap>
}

impl Command for SpawnLevel {
    fn apply(self, world: &mut World) {
        world.run_system_once_with(self, spawn_tmx);
    }
}

fn spawn_tmx(
    In(config): In<SpawnLevel>,
    mut commands: Commands,
    asset_server: Res<AssetServer>) {
    // commands.spawn(TiledMapBundle {
    //     tiled_map: config.map_handle,
    //     ..default()
    // });
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World, asset_server: AssetServer) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    // let map_handle = asset_server.load("levels/dungeon.tmx");
    // SpawnLevel { map_handle }.apply(world);
    SpawnPlayer { max_speed: 400.0 }.apply(world);
}
