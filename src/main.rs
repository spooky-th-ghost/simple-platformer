use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use heron::{prelude::*, Gravity, PhysicsPlugin};
use std::time::Duration;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(Gravity::from(Vec3::new(0.0, -500., 0.0)))
        .add_startup_system(spawn)
        .add_system(physics::detection::handle_ground_detection)
        .add_system(physics::detection::handle_left_wall_detection)
        .add_system(physics::detection::handle_right_wall_detection)
        .add_system(jump)
        .add_system(movement)
        .add_system(handle_busy)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Busy(pub Timer);

impl Busy {
    pub fn finished(&self) -> bool {
        self.0.finished()
    }

    pub fn tick(&mut self, delta: Duration) {
        self.0.tick(delta);
    }
}

fn handle_busy(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Busy)>
) {
    for (entity, mut busy) in &mut query {
        busy.tick(time.delta());
        if busy.finished() {
            println!("Removing Finished Timer");
            commands.entity(entity).remove::<Busy>();
        }
    }
}

fn jump(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        Entity,
        &mut Velocity, 
        Option<&physics::components::Grounded>,
        Option<&physics::components::LeftWall>,
        Option<&physics::components::RightWall>
        ), 
        (With<Player>,
         Without<Busy>
         )>,
) {
    for (entity, mut velocity, grounded, left_wall, right_wall) in &mut player_query {
        let mut x = 0.;
        let mut y = 0.;

        if input.pressed(KeyCode::Space) {
            if left_wall.is_some() {
                x = 200.;
                y = 200.;
            }

            if right_wall.is_some() {
                x = -200.;
                y = 200.;
            }

            if grounded.is_some() {
                y = 250.;
            }

            if x != 0. {velocity.linear.x = x}
            if y != 0. {velocity.linear.y = y}
            if left_wall.is_some() || right_wall.is_some() {
                commands.entity(entity).insert(Busy(Timer::from_seconds(0.25, false)));
            }
        }
    }
}

fn movement(
    input: Res<Input<KeyCode>>, 
    mut player_query: 
        Query<(
            &mut Velocity,
            Option<&physics::components::LeftWall>,
            Option<&physics::components::RightWall>
            ),
            (With<Player>,
             Without<Busy>)>
) {
    for (mut velocity, right_wall, left_wall) in &mut player_query {
        let mut x = 0.;

        if input.pressed(KeyCode::A) && right_wall.is_none(){
            x = -100.;
        }

        if input.pressed(KeyCode::D) && left_wall.is_none(){
            x = 100.;
        }

        if x != 0. {
            velocity.linear.x = x;
        }
    }
}

fn spawn(mut commands: Commands) {
    commands
        // Spawn any bundle of your choice. Only make sure there is a `GlobalTransform`
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::ONE * 50.),
                ..default()
            },
            transform: Transform::from_xyz(0., 100., 0.),
            ..default()
        })
        // Make it a rigid body
        .insert(RigidBody::Dynamic)
        // Attach a collision shape
        .insert(CollisionShape::Sphere { radius: 25.0 })
        // Optionally add other useful components...
        .insert(RotationConstraints::lock())
        .insert(
            CollisionLayers::none()
                .with_group(physics::detection::Layer::Player)
                .with_mask(physics::detection::Layer::World),
        )
        .insert(Velocity::default())
        .insert(Player)
        .insert(Name::new("Player"))
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::ONE * 6.),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::Y * -28.),
                    ..default()
                })
                .insert(physics::components::GroundDetector(Timer::from_seconds(0.1, false)))
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(3.0, 3.0, 1.0),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(physics::detection::Layer::GroundDetector)
                        .with_mask(physics::detection::Layer::World),
                )
                .insert(Name::new("Ground Detector"));

            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::ONE * 6.),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::X * -28.),
                    ..default()
                })
                .insert(physics::components::LeftWall)
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(3.0, 3.0, 1.0),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(physics::detection::Layer::LeftWallDetector)
                        .with_mask(physics::detection::Layer::World),
                )
                .insert(physics::components::LeftWallDetector)
                .insert(Name::new("Left Wall Detector"));

            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::GREEN,
                        custom_size: Some(Vec2::ONE * 6.),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::X * 28.),
                    ..default()
                })
                .insert(physics::components::RightWall)
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(3.0, 3.0, 1.0),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(physics::detection::Layer::RightWallDetector)
                        .with_mask(physics::detection::Layer::World),
                )
                .insert(physics::components::RightWallDetector)
                .insert(Name::new("Right Wall Detector"));
        });

    let floor_masks = [physics::detection::Layer::Player, physics::detection::Layer::GroundDetector, physics::detection::Layer::RightWallDetector, physics::detection::Layer::LeftWallDetector];

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ALICE_BLUE,
                custom_size: Some(Vec2::new(500., 10.)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(250., 5., 0.5),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(physics::detection::Layer::World)
                .with_masks(floor_masks.clone()),
        )
        .insert(Name::new("Floor"));

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ALICE_BLUE,
                custom_size: Some(Vec2::new(10., 500.)),
                ..default()
            },
            transform: Transform::from_xyz(-50., 0., 0.),
            ..default()
        })
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(5., 250., 0.5),
            border_radius: None,
        })
        .insert(
            CollisionLayers::none()
                .with_group(physics::detection::Layer::World)
                .with_masks(floor_masks.clone()),
        )
        .insert(Name::new("Floor"));
    commands.spawn_bundle(Camera2dBundle::default());
}
