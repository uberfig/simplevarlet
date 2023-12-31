use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::na::DimName;
// use bevy_rapier2d::na::U4;
// use std::time::Duration;

use crate::Player;

pub struct PixelPhysics;

impl Plugin for PixelPhysics {
    fn build(&self, app: &mut App) {
        app.insert_resource(PhysId(0))
            .add_systems(Update, (physics_tick, spawn_object, run_spawns))
            .add_systems(Startup, setup)
            .register_type::<PhysicsItem>()
			.register_type::<ItemSpawner>();
    }
}

#[derive(Resource)]
pub struct PhysId(pub usize);

// #[derive(Resource)]
// pub struct PhysList(pub Vec<>);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PhysicsItem {
    id: usize,
    pub position_old: Vec2,
    pub position: Vec2,
    pub velocity: Vec2, //in m/s
    pub acceleration: Vec2,
    pub mass: f32, //in kg
    pub radius: f32,
}

impl PhysicsItem {
    pub fn tick(&mut self, delta: f32) -> &Self {
        self.velocity = self.position - self.position_old;
        self.position_old = self.position;
        self.position = self.position + self.velocity + self.acceleration * delta * delta;
        self.acceleration = Vec2 { x: 0.0, y: 0.0 };
        return self;
    }

    pub fn accelerate(&mut self, accel: Vec2) -> &Self {
        self.acceleration += accel;
        return self;
    }

    // pub fn id(&mut self) -> usize {
    // 	return self.id;
    // }

    pub fn new(position: Vec2, id: &mut ResMut<PhysId>, radius: f32) -> Self {
        id.0 = id.0 + 1;
        Self {
            position_old: position,
            position: position,
            velocity: Vec2 { x: 0.0, y: 0.0 },
            acceleration: Vec2 { x: 0.0, y: 0.0 },
            mass: 0.0,
            id: id.0 - 1,
            radius: radius,
        }
    }
}

fn apply_constraints(mut items: Query<&mut PhysicsItem>) {
    let container_pos = Vec2 { x: 0.0, y: 0.0 };
    let radius: f32 = 150.0;
    // let obj_rad: f32 = 10.0;
    for mut physobj in &mut items {
        let to_obj = physobj.position - container_pos;
        let dist = to_obj.length();

        if dist > (radius - physobj.radius) {
            let n = to_obj / dist;
            let newpos = container_pos + n * (radius - physobj.radius);
            physobj.position = newpos;
            // info!("outisde of radius pos with newpos {:?}", newpos);
        }
    }
}

fn apply_gravity(
    // time: &Res<Time>,
    mut items: Query<&mut PhysicsItem>,
) {
    let gravity = Vec2 { x: 0.0, y: -1000.0 };
    // let gravity = Vec2 { x: 0.0, y: -9.8 * time.delta().as_secs_f32() * time.delta().as_secs_f32() };
    for mut physobj in &mut items {
        physobj.accelerate(gravity);
    }
}

fn update_postions(delta: f32, mut items: Query<&mut PhysicsItem>) {
    for mut physobj in &mut items {
        physobj.tick(delta);
    }
}

fn solve_collisions(
    // time: &Res<Time>,
    mut items: Query<&mut PhysicsItem>,
) {
    let mut combinations = items.iter_combinations_mut();
    while let Some([mut a1, mut a2]) = combinations.fetch_next() {
        if a1.id == a2.id {
            continue;
        }

        let collision_axis = a1.position - a2.position;
        let dist = collision_axis.length();
        let min_dist = a1.radius + a2.radius;
        if dist < min_dist {
            let n = collision_axis / dist;
            let delta = min_dist - dist;
            a1.position += 0.5 * delta * n;
            a2.position -= 0.5 * delta * n;
        }
    }
}

fn update_transforms(mut items: Query<(&mut PhysicsItem, &mut Transform)>) {
    for (physobj, mut transform) in &mut items {
        let z = transform.translation.z;
        transform.translation = Vec3 {
            x: physobj.position.x,
            y: physobj.position.y,
            z: z,
        };
    }
}

fn physics_tick(
    time: Res<Time>,
    mut set: ParamSet<(
        Query<&mut PhysicsItem>,
        Query<(&mut PhysicsItem, &mut Transform)>,
    )>,
) {
    let delta = time.delta().as_secs_f32();

    let substeps: usize = 4;
    let dt = delta / substeps as f32;

    for _ in 0..substeps {
        apply_gravity(set.p0());
        apply_constraints(set.p0());
        solve_collisions(set.p0());
        update_postions(dt, set.p0());
    }

    update_transforms(set.p1());
}

fn spawn_object(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    mut id: ResMut<PhysId>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = player.single(); //panics if theres more than one that matches the query, get single is recoverable

    // let texture = asset_server.load("textures/me.png");

    // commands.spawn((
    //     SpriteBundle {
    //         sprite: Sprite {
    //             custom_size: Some(Vec2::new(20.0, 20.0)),
    //             ..default()
    //         },
    //         texture,
    //         transform: *player_transform,
    //         ..default()
    //     },
    //     PhysicsItem::new(player_transform.translation.xy()),
    //     Name::new("PhysItem"),
    // ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: *player_transform,
            ..default()
        },
        PhysicsItem::new(player_transform.translation.xy(), &mut id, 10.0),
        Name::new("PhysItem"),
    ));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(150.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::GRAY)),
        transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
        ..default()
    });
    commands.spawn((
        ItemSpawner {
            postion: Vec2 { x: 0.1, y: 90.0 },
            timer: Timer::from_seconds(0.04, TimerMode::Once),
            step: 1,
            count: 0,
            max: 400,
        },
        Name::new("Spawner"),
    ));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct ItemSpawner {
    postion: Vec2,
    timer: Timer,
    step: u16,
    count: usize,
    max: usize,
}

fn run_spawns(
    mut commands: Commands,
    time: Res<Time>,
    mut items: Query<&mut ItemSpawner>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut id: ResMut<PhysId>,
) {
    for mut spawner in &mut items {
		spawner.timer.tick(time.delta());
        if spawner.count >= spawner.max {
            continue;
        }
        

        if spawner.timer.finished() {
			spawner.count += 1;
			spawner.timer.reset();
            // info!("time finished");
            let wrap = 512;
            spawner.step = spawner.step + 1;

            if spawner.step == wrap {
                spawner.step = 1
            }

            let radius: f32 = 1.4 * (spawner.step % 4) as f32 + 3.0;

            let _obj =
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::hsl((spawner.step % 360) as f32, 70.0, 66.0))),
                    transform: Transform {
                        translation: Vec3 {
                            x: spawner.postion.x,
                            y: spawner.postion.y,
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                },
                PhysicsItem::new(spawner.postion, &mut id, radius),
                Name::new("PhysItem"),
            ));
        }
    }
}
