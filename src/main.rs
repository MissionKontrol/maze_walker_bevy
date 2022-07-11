#![allow(unused)]

use bevy::prelude::*;
use maze_walker::*;

//#Region       --- Asset Constants

    const PLAYER_SPRITE: &str = "actors/runner_1_32.png";
    const PLAYER_SIZE: (f32, f32) = (32.,32.);

    const MAP_SPRITE: &str = "mazes/maze(1).png";
    const MAP_SIZE: (f32, f32) = (41.,41.);

//#End Region   --- Asset Constants


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GreetPlugin)
        .run();
}

#[derive(Component)]
struct Actor;

#[derive(Component)]
struct Name(String);

fn add_actors(mut commands: Commands) {
    commands.spawn().insert(Actor).insert(Name("Epic Hero".to_string()));
    println!("Greetings from add_actors");
}

fn greet_from_actors(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Actor>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            print!("Hello from: {}", name.0);
        }
    }
}

pub struct GreetPlugin;

impl Plugin for GreetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_startup_system(setup_system)
        .add_startup_system(add_actors)
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

struct GreetTimer(Timer);