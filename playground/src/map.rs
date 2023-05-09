use bevy::prelude::*;
use rand::Rng;
use TileType::*;

pub (super) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_map)
        ;
    }
}

#[derive(Component)]
pub struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType
}

#[derive(Copy, Clone)]
pub enum TileType {
    Floor,
    Wall
}

fn spawn_map(
    mut commands: Commands
) {
    let mut rng = rand::thread_rng();

    for x in 0..30 {
        for y in 0..30 {
            let tile_type = match rng.gen_bool(0.7) {
                true => Floor,
                false => Wall
            };

            let color = match tile_type {
                Floor => Color::rgba_u8(196, 164, 132, 255),
                Wall => Color::rgba_u8(101, 67, 33, 255),
            };

            commands.spawn((
                Tile {
                    x,
                    y,
                    tile_type
                },
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(32.0)),
                        color,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(x as f32 * 32.0, y as f32 * 32.0, 0.0)),
                    ..default()
                }
                ));
        }
    }
}