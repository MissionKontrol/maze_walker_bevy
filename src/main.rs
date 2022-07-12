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

    let maze = Maze::new(
        image.dimensions(),
        &pixel_list,
    );

    let entrances = maze.find_start();
    println!("Entrances {:?} {:?}", entrances.get_start(), entrances.get_end());
    let path = maze.solve_maze( &entrances.get_start(), &entrances.get_end());
    let my_path = path_to_points_system(path);

    App::new()
        .insert_resource(MapResource(my_path))
        .insert_resource(MapIndex(0))
        .insert_resource(WindowDescriptor{
            title: "Rust Mazer".to_string(),
            width:598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .run();
}

#[derive(Default, Debug)]
struct MapResource(Vec<Point>);

#[derive(Debug)]
struct MapIndex(usize);

#[derive(Component)]
struct Actor;

#[derive(Component)]
struct Name(String);

pub struct StartupPlugin;

struct MoveTimer(Timer);

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(MoveTimer(Timer::from_seconds(0.1, true)))
        .add_startup_system(setup_system)
        .add_startup_system(actor_setup_system)
        .add_system(move_actor_system);
    }
}

fn move_actor_system(map: Res<MapResource>, mut map_index: ResMut<MapIndex>,
    time: Res<Time>, mut timer: ResMut<MoveTimer>,
    mut query: Query<(&mut Transform, With<Actor>)>) {
        if timer.0.tick(time.delta()).just_finished() {
            println!("{:?}", map.0[map_index.0]);
            let mut x = map.0[map_index.0].x as f32;
            let mut y = map.0[map_index.0].y as f32;
            x = x as f32 * MAP_SCALE as f32 - 200.;
            y = 200. - y as f32 * MAP_SCALE as f32;
    
            if map_index.0 < (map.0.len() - 1) {
                map_index.0 += 1;
            }
            else {
                map_index.0 = 0;
            }
    
            for (mut transform, actor) in query.iter_mut() {
                dbg!(x);
                dbg!(y);
                if actor {
                    transform.translation = Vec3::new(x ,y ,2.0);
                }
            }
        }
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(990, 108));

    // map
    commands.spawn_bundle(SpriteBundle {
        texture:  asset_server.load(MAP_SPRITE),
        transform: Transform {
            scale: Vec3::new(MAP_SCALE,MAP_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn actor_setup_system(mut commands: Commands, asset_server: Res<AssetServer>, map: Res<MapResource>) {
    let start = map.0[0];

    let x = start.x as f32 * MAP_SCALE as f32 - 200.;
    let y = 200. - start.y as f32 * MAP_SCALE as f32;

    commands.spawn_bundle(SpriteBundle {
        texture:  asset_server.load(PLAYER_SPRITE),
            transform: Transform {
                scale: Vec3::new(ACTOR_SCALE,ACTOR_SCALE, 1.),
                translation: Vec3::new(x,y,2.0),
                ..Default::default()
            },
            ..Default::default()})
        .insert(Actor)
        .insert(Name("Eric Hero".to_string()));
}

fn path_to_points_system(path: Path) -> Vec<Point> {
    let mut my_path: Vec<Point> = Vec::new();
    for point in path.into_iter() {
        my_path.push(**point);
    }
    my_path
}