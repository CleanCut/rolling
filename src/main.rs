use bevy::{
    input::gamepad::{AxisSettings, GamepadSettings},
    prelude::*,
    render::texture::ImageSettings,
};
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(WindowDescriptor {
            title: "Rolling Game".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        // .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(setup)
        .add_system(movement)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Spawn the player
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("ball_blue_large.png"),
            ..Default::default()
        })
        .insert(Collider::ball(32.0))
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 5.0,
        })
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .build(),
        })
        .insert(Player);
}

const MOVE_FORCE: f32 = 1500.0;

fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    let (action_state, mut external_force) = query.single_mut();
    let axis_vector = action_state.clamped_axis_pair(Action::Move).unwrap().xy();
    external_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();
}
