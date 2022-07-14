use bevy::prelude::*;
use maze_walker::*;
mod breadcrumb;
use breadcrumb::*;
mod actor;
use actor::*;
mod mouse;
use mouse::*;

//#region       --- Asset Constants
const ACTOR_SPRITE: &str = "actors/runner_1_32.png";
// const PLAYER_SIZE: (f32, f32) = (32.,32.);
const ACTOR_SCALE: f32 = 0.5;
const MAP_SPRITE: &str = "mazes/maze(12).png";
const FQ_MAP_SPRITE: &str = "assets/mazes/maze(12).png";
// const MAP_SIZE: (f32, f32) = (41.,41.);
const MAP_SCALE: f32 = 10.;
const MAP_OFFSET: f32 = 500.;
const BREADCRUMB_ATLAS_END: &str = "diamond 4x4.png";
const BREADCRUMB_ATLAS_START: &str = "emerald-115-Sheet.png";
const BREADCRUMB_GRID_X: f32 = 115.;
const BREADCRUMB_GRID_Y: f32 = 115.;
const BREADCRUMB_COLS: usize = 4;
const BREADCRUMB_ROWS: usize = 4;
//#endregion   --- Asset Constants

//#region       --- Resources
pub struct GameTextures {
    breadcrumb_end: Handle<TextureAtlas>,
    breadcrumb_start: Handle<TextureAtlas>,
}
//#endregion     --- Resources

//#region       --- Components
#[derive(Default, Debug)]
struct MapPath(Vec<Point>);

#[derive(Debug)]
struct MapIndex(usize);

pub struct MapMaze(Maze);

#[derive(Component)]
pub struct Actor;

#[derive(Component)]
struct Name(String);

#[derive(Component, Default)]
pub struct ActorPathState {
    current_location: Point,
    visited: Vec<Point>,
    path: Vec<Point>,
    start: Point,
    end: Point,
}

#[derive(Component)]
pub struct ActorPathGoal(Point);

pub struct MoveTimer(Timer);

//#endregion  --- Components

fn main() {
    let maze = get_maze();
    let maze_dimensions = maze.get_dimensions();
    let (width, height) = (
        maze_dimensions.width as f32 * MAP_SCALE,
        maze_dimensions.height as f32 * MAP_SCALE,
    );

    App::new()
        .insert_resource(MapMaze(maze))
        .insert_resource(WindowDescriptor {
            title: "Rust Mazer".to_string(),
            width,
            height,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(ActorPlugin)
        .run();
}

fn get_maze() -> Maze {
    let image = Pnger::new(&FQ_MAP_SPRITE);
    let pixel_list = PixelList::new(&image.get_bytes(), image.dimensions());

    Maze::new(image.dimensions(), &pixel_list)
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveTimer(Timer::from_seconds(0.1, true)))
            .add_startup_system(setup_system)
            .add_startup_system(breadcrumb_setup_system)
            .add_system(cursor_position)
            .add_system(animate_breadcrumb_system);
    }
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //#region           --- Window setup
    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(990, 108));
    //#endregion        --- Window setup

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
