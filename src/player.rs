use crate::AsciiSheet;
use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Component)]
pub struct Player;

fn player_movement(
    mut query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (_player, mut transform) = query.single_mut();
    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += 0.5 * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= 0.5 * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= 0.5 * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += 0.5 * time.delta_seconds();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite,
            texture_atlas: ascii.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Name::new("Player"));
}
