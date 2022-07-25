use crate::ascii::{spawn_ascii_sprite, AsciiSheet};
use crate::tilemap::TileCollider;
use crate::TITLE_SIZE;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Inspectable)]
pub struct Player {
    pub speed: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(camera_follow.after("movement"))
            .add_system(player_movement.label("movement"));
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        1,
        Color::rgb(0.3, 0.3, 0.9),
        Vec3::new(2.0 * TITLE_SIZE, -2.0 * TITLE_SIZE, 900.0),
    );
    commands
        .entity(player)
        .insert(Player { speed: 3.0 })
        .insert(Name::new("Player"));
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn player_movement(
    mut query: Query<(&Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = query.single_mut();
    let mut y_delta = 0.0;
    let mut x_delta = 0.0;

    if keyboard.pressed(KeyCode::W) {
        y_delta += player.speed * TITLE_SIZE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::S) {
        y_delta -= player.speed * TITLE_SIZE * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::A) {
        x_delta -= player.speed * TITLE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        x_delta += player.speed * TITLE_SIZE * time.delta_seconds();
    }

    let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target
    }

    let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
    if wall_collision_check(target, &wall_query) {
        transform.translation = target
    }
}

fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TITLE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TITLE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}
