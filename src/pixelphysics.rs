use bevy::prelude::*;
use std::time::Duration;

use crate::{Player, newme::Newme};


pub struct PixelPhysics;

impl Plugin for PixelPhysics {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (physics_tick, spawn_object))
            .register_type::<PhysicsItem>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PhysicsItem {
    pub position_old: Vec2,
    pub position: Vec2,
    pub velocity: Vec2, //in m/s
    pub acceleration: Vec2,
    pub mass: f32, //in kg
}

impl PhysicsItem {
    pub fn tick(&mut self, delta: Duration) -> &Self {
        self.velocity = self.position - self.position_old;
        self.position_old = self.position;
        self.position = self.position + self.velocity + self.acceleration * delta.as_secs_f32() * delta.as_secs_f32();
        return self;
    }

    pub fn accelerate(&mut self, accel: Vec2) -> &Self {
        self.acceleration += accel;
        return self;
    }

    pub fn new(position: Vec2) -> Self {
        Self { 
            position_old: position, 
            position: position,
            velocity: Vec2 { x: 0.0, y: 0.0 },
            acceleration: Vec2 { x: 0.0, y: 0.0 },
            mass: 0.0,
        }
    }
}

fn apply_constraints(
    mut commands: Commands,
    mut items: Query<(&mut Transform, &mut PhysicsItem)>,
) {
    let container_pos = Vec2 {x: 0.0, y: 0.0};
    let radius: f32 = 400.0;
    for (mut transform, mut physobj) in &mut items {
        let to_obj = physobj.position - container_pos;
        let dist = to_obj.length();
        if (dist > radius - 50.0) {
            let n = to_obj / dist;
            physobj.position = container_pos + n * (dist - 50.0);
        }
    }
}

fn physics_tick(
    mut commands: Commands,
    time: Res<Time>,
    mut items: Query<(&mut Transform, &mut PhysicsItem)>,
) {
    for (mut transform, mut physobj) in &mut items {
        physobj.accelerate(Vec2 { x: 0.0, y: -9.8 * time.delta().as_secs_f32() * 100.0 });
        physobj.tick(time.delta());
        let z = transform.translation.z;
        transform.translation = Vec3 {x: physobj.position.x, y: physobj.position.y, z: z };
    }
    apply_constraints(commands, items);
}

fn spawn_object(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = player.single(); //panics if theres more than one that matches the query, get single is recoverable

    let texture = asset_server.load("textures/me.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            texture,
            transform: *player_transform,
            ..default()
        },
        PhysicsItem::new(player_transform.translation.xy()),
        Name::new("PhysItem"),
    ));
}