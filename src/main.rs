#![allow(unused)]

use bevy::prelude::*;
use maze_walker::*;

//#Region       --- Asset Constants

    const PLAYER_SPRITE: &str = "actors/runner_1_32.png";
    const PLAYER_SIZE: (f32, f32) = (32.,32.);

    const MAP_SPRITE: &str = "mazes/maze(1).png";
    const FQ_MAP_SPRITE: &str = "assets/mazes/maze(1).png";
    const MAP_SIZE: (f32, f32) = (41.,41.);

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

    let mut my_path: Vec<Point> = Vec::new();
    for point in path.into_iter() {
        my_path.push(**point);
    }



    App::new()
        .insert_resource(MapResource(my_path))
        .insert_resource(MapIndex(0))
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

fn add_actors(mut commands: Commands) {
    commands.spawn().insert(Actor).insert(Name("Eric Hero".to_string()));
    println!("Greetings from add_actors");
}

fn greet_from_actors(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Actor>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            print!("Hello from: {}", name.0);
        }
    }
}

fn map_info_from_actors(map: Res<MapResource>, map_index: ResMut<MapIndex>,
    query: Query<&Name, With<Actor>>) {
        println!("{:?}", map);
    }

fn print_index(map_index: Res<MapIndex>) {
    println!("{:?}", map_index);
}

fn get_path_length(map: Res<MapResource>) {
    println!("{:?}", map);
}

pub struct StartupPlugin;
struct GreetTimer(Timer);


impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(setup_system)
        .add_startup_system(add_actors)
        .add_system(map_info_from_actors)
        .add_system(greet_from_actors);
    }
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    window.set_position(IVec2::new(990, 108));


    const ACTOR_SCALE: f32 = 0.5;
    commands.spawn_bundle(SpriteBundle {
        texture:  asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            scale: Vec3::new(ACTOR_SCALE,ACTOR_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });


    const MAP_SCALE: f32 = 10.;
    commands.spawn_bundle(SpriteBundle {
        texture:  asset_server.load(MAP_SPRITE),
        transform: Transform {
            scale: Vec3::new(MAP_SCALE,MAP_SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
