use bevy::prelude::*;
use maze_walker::*;

//#Region       --- Asset Constants

const PLAYER_SPRITE: &str = "actors/runner_1_32.png";
// const PLAYER_SIZE: (f32, f32) = (32.,32.);
const ACTOR_SCALE: f32 = 0.5;

const MAP_SPRITE: &str = "mazes/maze(3).png";
const FQ_MAP_SPRITE: &str = "assets/mazes/maze(3).png";
// const MAP_SIZE: (f32, f32) = (41.,41.);
const MAP_SCALE: f32 = 10.;

//#End Region   --- Asset Constants

fn main() {
    let image = Pnger::new(&FQ_MAP_SPRITE);
    let pixel_list = PixelList::new(&image.get_bytes(), image.dimensions());

    let maze = Maze::new(image.dimensions(), &pixel_list);

    let (start, end) = maze.find_start();
    println!("Entrances {:?} {:?}", start, end);
    let path = maze.solve_maze(&start, &end);
    let my_path = path_to_points_system(path);

    App::new()
        .insert_resource(MapPath(my_path))
        .insert_resource(MapMaze(maze))
        .insert_resource(MapIndex(0))
        .insert_resource(MazeEntrances)
        .insert_resource(WindowDescriptor {
            title: "Rust Mazer".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .run();
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveTimer(Timer::from_seconds(0.1, true)))
            .add_startup_system(setup_system)
            .add_startup_system(actor_setup_system)
            .add_startup_system(breadcrumb_setup_system)
            .add_system(animate_breadcrumb_system)
            .add_system(actor_solve_next_step_system)
            .add_system(move_actor_system);
    }
}

#[derive(Default, Debug)]
struct MapPath(Vec<Point>);

#[derive(Debug)]
struct MapIndex(usize);

struct MapMaze(Maze);

#[derive(Component)]
struct Actor;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct MazeEntrances (Vec<Point>);

#[derive(Component, Default)]
struct ActorPathState {
    current_location: Point,
    visited: Vec<Point>,
    path: Vec<Point>,
    start: Point,
    end: Point,
}

#[derive(Component)]
struct BreadCrumbs (Vec<Point>);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

struct MoveTimer(Timer);

fn breadcrumb_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
 ){
    let texture_handle = asset_server.load("diamond 4x4.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(115.0, 115.0), 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)));

}

fn animate_breadcrumb_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
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

fn actor_solve_next_step_system(
    maze: Res<MapMaze>,
    mut query: Query<(&mut ActorPathState, With<Actor>)>,
 ){
    for (mut path_state, _) in query.iter_mut() {
        if path_state.current_location != path_state.end {
            let connections = maze.0.get_point_connections(&path_state.current_location);
            let next_point = connections
                .iter()
                .find(|x| !visited(*x, &path_state.visited));

            if let Some(point) = next_point {
                path_state.visited.push(**point);
                path_state.path.push(**point);
                path_state.current_location = **point;
            } else {
                path_state.current_location = path_state.path.pop().unwrap();
            }
        }
        else {
            path_state.current_location = path_state.start;
            path_state.visited.clear();
            path_state.path.clear();
        }
    }
}

fn visited(point: &Point, visited: &Vec<Point>) -> bool {
    if let Some(_) = visited.iter().find(|x| *x == point) {
        true
    } else {
        false
    }
}

fn move_actor_system(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut query: Query<(&mut Transform, &ActorPathState, With<Actor>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut transform, path_state, _) in query.iter_mut() {
            let ( x, y) = path_state.current_location.to_tuple();
            let x = x as f32 * MAP_SCALE as f32 - 200.;
            let y = 200. - y as f32 * MAP_SCALE as f32;
            transform.translation = Vec3::new(x, y, 2.0);
        }
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(990, 108));

    // map
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(MAP_SPRITE),
        transform: Transform {
            scale: Vec3::new(MAP_SCALE, MAP_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn actor_setup_system(mut commands: Commands, asset_server: Res<AssetServer>, map: Res<MapMaze>) {
    let (start, end) = map.0.find_start();
    dbg!(start);
    dbg!(end);

    let x = start.x as f32 * MAP_SCALE as f32 - 200.;
    let y = 200. - start.y as f32 * MAP_SCALE as f32;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform {
                scale: Vec3::new(ACTOR_SCALE, ACTOR_SCALE, 1.),
                translation: Vec3::new(x, y, 2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Actor)
        .insert(Name("Eric Hero".to_string()))
        .insert(ActorPathState {
            current_location: start,
            visited: Vec::new(),
            path: Vec::new(),
            start,
            end,
        });
}

fn path_to_points_system(path: Path) -> Vec<Point> {
    let mut my_path: Vec<Point> = Vec::new();
    for point in path.into_iter() {
        my_path.push(**point);
    }
    my_path
}
