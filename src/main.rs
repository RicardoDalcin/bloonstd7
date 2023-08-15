use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

#[derive(Component)]
enum Direction {
    Right,
    Left,
}

const BALLOON_SIZE: f32 = 50.;
const BALLOON_SPEED: f32 = 150.;

fn start_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2dBundle::default());

    let window = window_query.get_single().unwrap();
    let width = window.width();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALLOON_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(-width / 2. + BALLOON_SIZE, 0., 0.)),
            ..default()
        },
        Direction::Right,
    ));
}

fn move_balloons(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width();

    for (mut balloon, mut transform) in &mut sprite_position {
        match *balloon {
            Direction::Right => transform.translation.x += BALLOON_SPEED * time.delta_seconds(),
            Direction::Left => transform.translation.x -= BALLOON_SPEED * time.delta_seconds(),
        }

        if transform.translation.x > width / 2. - BALLOON_SIZE {
            *balloon = Direction::Left;
        } else if transform.translation.x < -width / 2. + BALLOON_SIZE {
            *balloon = Direction::Right;
        }
    }
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_balloons(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut timer: ResMut<SpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let window = window_query.get_single().unwrap();
        let width = window.width();

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BALLOON_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(
                    -width / 2. + BALLOON_SIZE,
                    0.,
                    0.,
                )),
                ..default()
            },
            Direction::Right,
        ));
    }
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_scene)
            .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Update, (move_balloons, spawn_balloons));
    }
}

fn main() {
    App::new().add_plugins((DefaultPlugins, GamePlugin)).run();
}
