use crate::*;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_set_to_stage(
            StartupStage::PostStartup,
            SystemSet::new().with_system(actor_setup_system),
        )
        .add_system(actor_solve_next_step_system)
        .add_system(move_actor_system)
        .add_system(breadcrumb_spawn_system);
    }
}

pub fn actor_solve_next_step_system(
    maze: Res<MapMaze>,
    mut query: Query<(&mut ActorPathState, With<Actor>)>,
) {
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
        } else {
            path_state.current_location = path_state.start;
            path_state.visited.clear();
            path_state.path.clear();
        }
    }
}

pub fn visited(point: &Point, visited: &Vec<Point>) -> bool {
    if let Some(_) = visited.iter().find(|x| *x == point) {
        true
    } else {
        false
    }
}

pub fn move_actor_system(
    time: Res<Time>,
    mut timer: ResMut<MoveTimer>,
    mut query: Query<(&mut Transform, &ActorPathState, With<Actor>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut transform, path_state, _) in query.iter_mut() {
            let (x, y) = path_state.current_location.to_tuple();
            let x = x as f32 * MAP_SCALE as f32 - 200.;
            let y = 200. - y as f32 * MAP_SCALE as f32;
            transform.translation = Vec3::new(x, y, 2.0);
        }
    }
}

pub fn actor_setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Res<MapMaze>,
) {
    let (start, end) = map.0.find_start();
    dbg!(start);
    dbg!(end);

    let x = start.x as f32 * MAP_SCALE as f32 - 200.;
    let y = 200. - start.y as f32 * MAP_SCALE as f32;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(ACTOR_SPRITE),
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
