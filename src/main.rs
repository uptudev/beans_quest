use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection, LdtkPlugin};
#[allow(unused_imports)]
use iyes_loopless::prelude::*;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            // uncomment for unthrottled FPS
            present_mode: bevy::window::PresentMode::AutoNoVsync,
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(setup_physics)
        .insert_resource(LevelSelection::Index(0))
        .add_system_set(SystemSet::on_enter(GameState::Next).with_system(use_my_assets))
        .add_system(print_ball_altitude)
        .run();
}

#[derive(Component)]
struct GameCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle::default(), 
        GameCamera
    ));

    commands.spawn(
        LdtkWorldBundle {
            ldtk_handle: asset_server.load("maps/test_level.ldtk"),
            ..Default::default()
        }
    );
}

fn use_my_assets() {
    //TODO something
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    AssetLoading,
    Next,
}

/*
 * A test of the Rapier physics system
 */
fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}