use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkWorldBundle;
use iyes_loopless::prelude::*;
use gamelibs::map::{LdtkPlugin, LdtkMap, LdtkMapBundle, LdtkMapConfig};

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            // uncomment for unthrottled FPS
            // present_mode: bevy::window::PresentMode::AutoNoVsync,
            title: "PLACEHOLDER".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        },
        ..default()
    }))
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Next)
        )
        .add_state(GameState::AssetLoading)
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_enter(GameState::Next).with_system(use_my_assets))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/test_level.ldtk"),
        ..Default::default()
    });
}

fn use_my_assets() {
    //TODO something
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next,
}