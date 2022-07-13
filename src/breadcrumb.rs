use crate::*;

#[derive(Component, Default)]
pub struct BreadCrumbs(Vec<Point>);

#[derive(Component)]
pub struct BreadCrumb;

#[derive(Component, Deref, DerefMut)]
pub struct BreadCrumbAnimationTimer(Timer);

pub fn breadcrumb_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut query: Query<(&ActorPathState, With<Actor>)>,
) {
    for (path, _) in query.iter_mut() {

    let (x,y) = path.current_location.to_tuple();
    let x = x as f32 * MAP_SCALE as f32 - 200.;
    let y = 200. - y as f32 * MAP_SCALE as f32;

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_textures.breadcrumb.clone(),
            transform: Transform {
                scale: Vec3::new(0.08, 0.08, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(BreadCrumb)
        .insert(BreadCrumbAnimationTimer(Timer::from_seconds(0.1, true)));
    }
}

pub fn breadcrumb_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(BREADCRUMB_ATLAS);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(BREADCRUMB_GRID_X, BREADCRUMB_GRID_Y),
        BREADCRUMB_COLS,
        BREADCRUMB_ROWS,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let game_textures = GameTextures {
        breadcrumb: texture_atlas_handle,
    };
    commands.insert_resource(game_textures);
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
            sprite.index = (sprite.index + 1) % (texture_atlas.textures.len() - 1);
            // skip last empty cell in atlas...fix
        }
    }
}
