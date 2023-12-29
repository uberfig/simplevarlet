use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::time::Duration;

use crate::Player;


pub struct PixelPhysics;

impl Plugin for PixelPhysics {
	fn build(&self, app: &mut App) {
		app
			.insert_resource(PhysId(0))
			.add_systems(Update, (physics_tick, spawn_object))
			.add_systems(Startup, setup)
			.register_type::<PhysicsItem>();
	}
}

#[derive(Resource)]
pub struct PhysId(pub usize);

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
		self.acceleration = Vec2 {x: 0.0, y:0.0};
		return self;
	}

	pub fn accelerate(&mut self, accel: Vec2) -> &Self {
		self.acceleration += accel;
		return self;
	}

	pub fn id(&mut self) -> usize {
		return self.id;
	}

	pub fn new(position: Vec2, mut id: ResMut<PhysId>,) -> Self {
		id.0 = id.0+1;
		Self { 
			position_old: position, 
			position: position,
			velocity: Vec2 { x: 0.0, y: 0.0 },
			acceleration: Vec2 { x: 0.0, y: 0.0 },
			mass: 0.0,
			id: id.0-1,
			radius: 10.0,
		}
	}
}

fn apply_constraints(
	mut items: Query<&mut PhysicsItem>,
) {
	let container_pos = Vec2 {x: 0.0, y: 0.0};
	let radius: f32 = 50.0;
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
	time: &Res<Time>,
	mut items: Query<&mut PhysicsItem>,
) {
	let gravity = Vec2 {x: 0.0, y: -1000.0};
	// let gravity = Vec2 { x: 0.0, y: -9.8 * time.delta().as_secs_f32() * time.delta().as_secs_f32() };
	for mut physobj in &mut items {
		physobj.accelerate(gravity);
	}
}

fn update_postions(
	time: &Res<Time>,
	mut items: Query<&mut PhysicsItem>,
) {
	let delta = time.delta().as_secs_f32();
	for mut physobj in &mut items {
		physobj.tick(delta);
	}
}

fn update_transforms(
	time: &Res<Time>,
	mut items: Query<(&mut PhysicsItem, &mut Transform)>,
) {
	for (mut physobj, mut transform) in &mut items {
		let z = transform.translation.z;
		transform.translation = Vec3 {x: physobj.position.x, y: physobj.position.y, z: z };
	}
}

fn physics_tick(
	time: Res<Time>,
	mut set: ParamSet<(
		Query<&mut PhysicsItem>,
		Query<(&mut PhysicsItem, &mut Transform)>,
	)>,
) {
	apply_gravity(&time, set.p0());
	apply_constraints(set.p0());	

	let mut binding = set.p0();
 	let mut combinations = binding.iter_combinations_mut();
	while let Some([mut a1, mut a2]) = combinations.fetch_next() {
		if a1.id == a2.id {
			continue;
		}

		let collision_axis = a1.position - a2.position;
		let dist = collision_axis.length();
		let mut max = a1.radius.max(a2.radius);
		max = max*2.0;
		if dist < max {
			let n = collision_axis / dist;
			let delta = max - dist;
			a1.position += 0.5 * delta * n;
			a2.position -= 0.5 * delta * n;
		}
    }

	update_postions(&time, set.p0());
	update_transforms(&time, set.p1());
}

fn spawn_object(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
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
		PhysicsItem::new(player_transform.translation.xy(), id),
		Name::new("PhysItem"),
	));
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {

	commands.spawn((
		SpriteBundle {
			sprite: Sprite {
				custom_size: Some(Vec2::new(4.0, 4.0)),
				..default()
			},
			transform: Transform {
				translation: Vec3 { x: 10.0, y: 0.0, z: 0.0},
				..default()
			},
			..default()
		},
		Name::new("helper"),
	));
	commands.spawn((
		SpriteBundle {
			sprite: Sprite {
				custom_size: Some(Vec2::new(4.0, 4.0)),
				..default()
			},
			transform: Transform {
				translation: Vec3 { x: -10.0, y: 0.0, z: 0.0},
				..default()
			},
			..default()
		},
		Name::new("helper"),
	));
	commands.spawn((
		SpriteBundle {
			sprite: Sprite {
				custom_size: Some(Vec2::new(4.0, 4.0)),
				..default()
			},
			transform: Transform {
				translation: Vec3 { x: 0.0, y: -10.0, z: 0.0},
				..default()
			},
			..default()
		},
		Name::new("helper"),
	));
	commands.spawn((
		SpriteBundle {
			sprite: Sprite {
				custom_size: Some(Vec2::new(4.0, 4.0)),
				..default()
			},
			transform: Transform {
				translation: Vec3 { x: 0.0, y: 10.0, z: 0.0},
				..default()
			},
			..default()
		},
		Name::new("helper"),
	));

	commands.spawn(MaterialMesh2dBundle {
		mesh: meshes.add(shape::Circle::new(50.).into()).into(),
		material: materials.add(ColorMaterial::from(Color::GRAY)),
		transform: Transform::from_translation(Vec3::new(0., 0., -0.5)),
		..default()
	});
}