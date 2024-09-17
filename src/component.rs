use bevy::prelude::*;

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct BoxEntity;

#[derive(Component)]
pub struct StartPoint;

#[derive(Component)]
pub struct EndPoint;

#[derive(Component)]
pub struct BoxDirection(pub Vec3);

#[derive(Component)]
pub struct Fireball;
