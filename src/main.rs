mod images;
use bevy::render::render_asset::RenderAssetUsages;
use images::*;

use bevy::prelude::*;
use bevy::render::texture::Image;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, generate_map)
        .run();
}

fn generate_map(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
) {
    let grid_size: u32 = 32;
    let grid_scale: f32 = 10.0;
    let outline_thickness: u32 = 6;

    let image = generate_grid_image_dynamic(grid_size, outline_thickness);
    let texture = Image::from_dynamic(
        image,
        true,
        
        RenderAssetUsages::RENDER_WORLD,
    );

    let image_handle: Handle<Image> = textures.add(texture);

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(grid_scale))
                .with_translation(Vec3::new(0.0, 0.0, 0.0)),
            texture: image_handle.clone(),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: true,
            stretch_value: 0.5,
        },
    ));
}
