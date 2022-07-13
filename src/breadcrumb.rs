use bevy::prelude::*;
use crate::*;

#[derive(Component)]
struct BreadCrumbs (Vec<Point>);

#[derive(Component, Deref, DerefMut)]
pub struct BreadCrumbAnimationTimer(Timer);

pub fn breadcrumb_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
 ){
    let texture_handle = asset_server.load(BREADCRUMB_ATLAS);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(115.0, 115.0), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                scale: Vec3::new(0.1, 0.1, 2.),
                translation: Vec3::new(0., 0., 2.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(BreadCrumbAnimationTimer(Timer::from_seconds(0.1, true)));
}

pub fn animate_breadcrumb_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut BreadCrumbAnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % (texture_atlas.textures.len() - 1);     // skip last empty cell in atlas...fix
        }
    }
}