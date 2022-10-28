use std::f32::consts::PI;

use bevy::{prelude::*, render::texture::ImageSettings};
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
        .add_system(win_condition)
        .add_system(collision_sounds)
        .run();
}

#[derive(Component)]
struct Player {
    id: usize,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Spawn the players
    spawn_player(0, Vec2::new(-100.0, 0.0), &mut commands, &asset_server);
    spawn_player(1, Vec2::new(100.0, 0.0), &mut commands, &asset_server);

    // Spawn the goal
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(450.0, -300.0, 0.0)),
            texture: asset_server.load("rolling/hole_large_end.png"),
            ..Default::default()
        })
        .insert(Collider::ball(8.0))
        .insert(Sensor)
        .insert(Goal);

    // Spawn the pieces
    spawn_piece(Vec2::new(150.0, 150.0), 0.0, &mut commands, &asset_server);
    spawn_piece(
        Vec2::new(-350.0, 50.0),
        PI * 0.5,
        &mut commands,
        &asset_server,
    );
    spawn_piece(Vec2::new(-150.0, -200.0), PI, &mut commands, &asset_server);
    spawn_piece(
        Vec2::new(200.0, -50.0),
        PI * 1.5,
        &mut commands,
        &asset_server,
    );
}

fn spawn_player(
    id: usize,
    location: Vec2,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let image = if id == 0 {
        "rolling/ball_blue_large.png"
    } else {
        "rolling/ball_red_large.png"
    };
    // Spawn the player
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(image),
            transform: Transform::from_translation(location.extend(1.0)),
            ..Default::default()
        })
        .insert(Collider::ball(32.0))
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(Restitution::coefficient(1.0))
        .insert(Damping {
            linear_damping: 0.6,
            angular_damping: 0.3,
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(DualAxis::left_stick(), Action::Move)
                .set_gamepad(Gamepad { id })
                .build(),
        })
        .insert(Player { id });
}

fn spawn_piece(
    location: Vec2,
    rotation: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("rolling/block_corner.png"),
            transform: Transform {
                translation: location.extend(0.0),
                rotation: Quat::from_rotation_z(rotation),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::round_triangle(
            Vec2::new(-23.0, -23.0),
            Vec2::new(-23.0, 23.0),
            Vec2::new(23.0, -23.0),
            0.05,
        ))
        .insert(RigidBody::Fixed)
        .insert(Restitution::coefficient(1.0));
}

const MOVE_FORCE: f32 = 1500.0;

fn movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
) {
    for (action_state, mut external_force) in query.iter_mut() {
        let axis_vector = action_state.clamped_axis_pair(Action::Move).unwrap().xy();
        external_force.force = axis_vector * MOVE_FORCE * time.delta_seconds();
    }
}

#[derive(Component)]
struct Goal;

fn win_condition(
    rapier_context: Res<RapierContext>,
    player_query: Query<(Entity, &Player)>,
    goal_query: Query<Entity, With<Goal>>,
) {
    let goal_entity = goal_query.single();
    for (player_entity, player) in player_query.iter() {
        if rapier_context.intersection_pair(goal_entity, player_entity) == Some(true) {
            println!("Player {} wins!", player.id);
        }
    }
}

fn collision_sounds(
    rapier_context: Res<RapierContext>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let mut just_collided = false;
    for pair in rapier_context.contact_pairs() {
        if pair.has_any_active_contacts() {
            just_collided = true;
        }
    }
    if just_collided {
        let sound = asset_server.load("impact/impactGlass_heavy_002.ogg");
        audio.play(sound);
    }
}
