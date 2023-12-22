use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture}, input::common_conditions::input_toggle_active,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{Money, Player};

pub struct NewMePlugin;

impl Plugin for NewMePlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_systems(Update, (spawn_thing, newme_lifetime))
            .register_type::<Newme>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Newme {
    pub lifetime: Timer,
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
            lifetime: Timer::from_seconds(10.0, TimerMode::Once),
        },
        Name::new("Newme"),
    ));
}