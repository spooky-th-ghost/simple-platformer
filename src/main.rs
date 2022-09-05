use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin};
use heron::{prelude::*, Gravity, PhysicsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(Gravity::from(Vec3::new(0.0, -500., 0.0)))
        .add_startup_system(spawn)
        .add_system(set_grounded)
        .add_system(jump)
        .add_system(movement)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct GroundDetector(pub Timer);

fn set_grounded(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    mut query: Query<&Parent, With<GroundDetector>>,
) {
    for event in events.iter() {
        for parent in &query {
            match event {
                CollisionEvent::Stopped(d1, d2) => {
                    let (l1, l2) = event.collision_layers();

                    if is_ground_detector(l1) && is_world(l2) {
                        let normals = d1.normals();
                        println!("{}", normals.len());
                        for n in d1.normals() {
                            println!("{}", n);
                        }
                        commands.entity(parent.get()).remove::<Grounded>();
                        println!("Player has left the ground");
                    } else if is_ground_detector(l2) && is_world(l1) {
                        let normals = d2.normals();
                        println!("{}", normals.len());
                        for n in d2.normals() {
                            println!("{}", n);
                        }
                        commands.entity(parent.get()).remove::<Grounded>();
                        println!("Player has left the ground");
                    }
                }
                CollisionEvent::Started(_, _) => {
                    let (l1, l2) = event.collision_layers();

                    if is_ground_detector(l1) && is_world(l2) {
                        commands.entity(parent.get()).insert(Grounded);
                        println!("Player has landed");
                    } else if is_ground_detector(l2) && is_world(l1) {
                        commands.entity(parent.get()).insert(Grounded);
                        println!("Player has landed");
                    }
                }
            }
        }
    }
}

fn jump(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Velocity, Option<&Grounded>), With<Player>>,
) {
    for (mut velocity, grounded) in &mut player_query {
        if let Some(_) = grounded {
            if input.pressed(KeyCode::Space) {
                println!("He Pressin");
                velocity.linear.y = 250.;
            }
        }
    }
}

fn movement(input: Res<Input<KeyCode>>, mut player_query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in &mut player_query {
        let mut x = 0.;

        if input.pressed(KeyCode::A) {
            x = -100.;
        }

        if input.pressed(KeyCode::D) {
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
                .with_group(Layer::Player)
                .with_mask(Layer::World),
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
                    transform: Transform::from_translation(Vec3::Y * -31.),
                    ..default()
                })
                .insert(GroundDetector(Timer::from_seconds(0.1, false)))
                .insert(RigidBody::Sensor)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(3.0, 3.0, 1.0),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::none()
                        .with_group(Layer::GroundDetector)
                        .with_mask(Layer::World),
                );
        });

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
                .with_group(Layer::World)
                .with_masks([Layer::Player, Layer::GroundDetector]),
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
                .with_group(Layer::World)
                .with_masks([Layer::Player, Layer::GroundDetector]),
        )
        .insert(Name::new("Floor"));
    commands.spawn_bundle(Camera2dBundle::default());
}
fn is_player(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Player)
}

fn is_world(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::World)
}

fn is_ground_detector(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::GroundDetector)
}
// Define your physics layers
#[derive(PhysicsLayer)]
enum Layer {
    World,
    Player,
    GroundDetector,
}
