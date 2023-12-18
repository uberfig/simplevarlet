use std::time::Duration;

use bevy::{
    prelude::*,
    render::{camera::ScalingMode},
};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Component)]
pub struct PhysicsItem {
    pub velocity: Vec2, //in m/s
    pub mass: f32, //in kg
}

impl PhysicsItem {
    pub fn tick(&mut self, delta: Duration) -> &Self {
        

        return self;
    }
}

#[derive(Component)]
pub struct Newme {
    pub lifetime: Timer,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) //nearest neighbor filtering by default for pixelart
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Ivy's secret project".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup) //run setup at the start of the program
        .add_systems(Update, (character_movement, spawn_thing, newme_lifetime)) //run every frame
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn(
    //     Camera2dBundle {
    //         camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::GREEN), },
    //         ..default() //takes the rest of the default props
    //     }
    // );
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    let texture = asset_server.load("textures/me.png");

    //tuple of the sprite and a player
    //when in a tuple, bevy treats it like a bundle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        let mut direction_vector = Vec2::new(0.0, 0.0);

        if input.pressed(KeyCode::W) {
            direction_vector.y += 1.0;
        }
        if input.pressed(KeyCode::S) {
            direction_vector.y -= 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction_vector.x += 1.0;
        }
        if input.pressed(KeyCode::A) {
            direction_vector.x -= 1.0;
        }
        let dirvec = direction_vector.normalize_or_zero();
        transform.translation.x += dirvec.x * movement_amount;
        transform.translation.y += dirvec.y * movement_amount;
    }
}

fn spawn_thing(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }
    let player_transform = player.single(); //panics if theres more than one that matches the query, get single is recoverable

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 to spawn, remaining money: ${:?}", money.0);
    } else {
        info!("not enough money, remaining money: ${:?}", money.0);
        return;
    }

    let texture = asset_server.load("textures/newme.png");

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
        Newme {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        }
    ));
}

fn newme_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut newme: Query<(Entity, &mut Newme)>,
    mut money: ResMut<Money>,
) {
    for (newme_entity, mut me) in &mut newme {
        me.lifetime.tick(time.delta());

        if me.lifetime.finished() {
            money.0 += 15.0;

            commands.entity(newme_entity).despawn();

            info!("Newme dissapear for $15, Current money: ${:?}", money.0);
        }
    }
}