use bevy::prelude::*;

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct RightWall;

#[derive(Component)]
pub struct LeftWall;

#[derive(Component)]
pub struct GroundDetector(pub Timer);

#[derive(Component)]
pub struct LeftWallDetector;

#[derive(Component)]
pub struct RightWallDetector;
