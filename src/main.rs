
mod newme;
mod pixelphysics;

use bevy::{
    prelude::*,
    render::camera::ScalingMode, input::common_conditions::input_toggle_active,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use newme::NewMePlugin;
use pixelphysics::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Resource)]
pub struct Money(pub f32);


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
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup) //run setup at the start of the program
        .add_systems(Update, (character_movement)) //run every frame
        .add_plugins(NewMePlugin)
        .add_plugins(PixelPhysics)
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
        // min_width: 256.0,
        // min_height: 144.0,
        min_width: 512.0,
        min_height: 288.0,
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
        Name::new("Player"),
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



