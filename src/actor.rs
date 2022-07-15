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
    mut query: Query<(&mut ActorPathState, &mut ActorPathGoal, With<Actor>)>,
) {
    print!(" ");
    for (mut path_state, mut goal, _) in query.iter_mut() {
        if path_state.current_location != goal.0 {
            let connections = maze
                .0
                .get_point_connections(&path_state.current_location)
                .unwrap();
            let next_point = connections
                .iter()
                .find(|x| !visited(*x, &path_state.visited));

            if let Some(point) = next_point {
                path_state.visited.push(**point);
                path_state.path.push(**point);
                path_state.current_location = **point;
            } else {
                path_state.path.pop();  // current location
                let current_location = path_state.path.pop().unwrap(); // last good location
                path_state.current_location = current_location;
                path_state.path.push(current_location);  // keep it on the path
            }
        } else {
            if path_state.current_location == path_state.start {
                goal.0 = path_state.end;
            } else {
                goal.0 = path_state.start;
            }
            path_state.visited.clear();
            path_state.path.clear();
            let current_location = path_state.current_location;
            path_state.path.push(current_location);
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
            let x = x as f32 * MAP_SCALE as f32 - MAP_OFFSET;
            let y = MAP_OFFSET - y as f32 * MAP_SCALE as f32;
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
    let end = Point{ x: 101, y: 99};
    
    let x = start.x as f32 * MAP_SCALE as f32 - MAP_OFFSET;
    let y = MAP_OFFSET - start.y as f32 * MAP_SCALE as f32;

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
        .insert(ActorPathGoal(end))
        .insert(ActorPathState {
            current_location: start,
            visited: Vec::new(),
            path: Vec::new(),
            start,
            end,
        });

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
        .insert(Name("Eric II Hero".to_string()))
        .insert(ActorPathGoal(end))
        .insert(ActorPathState {
            current_location: end,
            visited: Vec::new(),
            path: Vec::new(),
            start,
            end,
        });
}
