use bevy::prelude::*;
use heron::prelude::*;

pub fn handle_ground_detection(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    query: Query<&Parent, With<super::components::GroundDetector>>,
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
                        commands.entity(parent.get()).remove::<super::components::Grounded>();
                        println!("Player has left the ground");
                    } else if is_ground_detector(l2) && is_world(l1) {
                        let normals = d2.normals();
                        println!("{}", normals.len());
                        for n in d2.normals() {
                            println!("{}", n);
                        }
                        commands.entity(parent.get()).remove::<super::components::Grounded>();
                        println!("Player has left the ground");
                    }
                }
                CollisionEvent::Started(_, _) => {
                    let (l1, l2) = event.collision_layers();

                    if is_ground_detector(l1) && is_world(l2) {
                        commands.entity(parent.get()).insert(super::components::Grounded);
                        println!("Player has landed");
                    } else if is_ground_detector(l2) && is_world(l1) {
                        println!("Player has landed");
                    }
                }
            }
        }
    }
}

pub fn handle_left_wall_detection(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    query: Query<&Parent, With<super::components::LeftWallDetector>>,
) {
    for event in events.iter() {
        for parent in &query {
            match event {
                CollisionEvent::Stopped(d1, d2) => {
                    let (l1, l2) = event.collision_layers();

                    if is_left_wall_detector(l1) && is_world(l2) {
                        commands.entity(parent.get()).remove::<super::components::LeftWall>();
                    } else if is_left_wall_detector(l2) && is_world(l1) {
                        commands.entity(parent.get()).remove::<super::components::LeftWall>();
                    }
                }
                CollisionEvent::Started(_, _) => {
                    let (l1, l2) = event.collision_layers();

                    if is_left_wall_detector(l1) && is_world(l2) {
                        commands.entity(parent.get()).insert(super::components::LeftWall);
                    } else if is_left_wall_detector(l2) && is_world(l1) {
                        commands.entity(parent.get()).insert(super::components::LeftWall);
                    }
                }
            }
        }
    }
}

pub fn handle_right_wall_detection(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    query: Query<&Parent, With<super::components::RightWallDetector>>,
) {
    for event in events.iter() {
        for parent in &query {
            match event {
                CollisionEvent::Stopped(d1, d2) => {
                    let (l1, l2) = event.collision_layers();

                    if is_right_wall_detector(l1) && is_world(l2) {
                        commands.entity(parent.get()).remove::<super::components::RightWall>();
                    } else if is_right_wall_detector(l2) && is_world(l1) {
                        commands.entity(parent.get()).remove::<super::components::RightWall>();
                    }
                }
                CollisionEvent::Started(_, _) => {
                    let (l1, l2) = event.collision_layers();

                    if is_right_wall_detector(l1) && is_world(l2) {
                        commands.entity(parent.get()).insert(super::components::RightWall);
                    } else if is_right_wall_detector(l2) && is_world(l1) {
                        commands.entity(parent.get()).insert(super::components::RightWall);
                    }
                }
            }
        }
    }
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

fn is_left_wall_detector(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::LeftWallDetector)
}

fn is_right_wall_detector(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::RightWallDetector)
}

// Define your physics layers
#[derive(PhysicsLayer, Clone)]
pub enum Layer {
    World,
    Player,
    GroundDetector,
    LeftWallDetector,
    RightWallDetector
}
