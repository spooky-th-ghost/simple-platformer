use bevy::prelude::*;
use heron::prelude::*;

pub fn handle_ground_detector(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    query: Query<&Parent, With<GroundDetector>>,
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

// Define your physics layers
#[derive(PhysicsLayer, Clone)]
enum Layer {
    World,
    Player,
    GroundDetector,
    LeftWallDetector,
    RightWallDetector
}
