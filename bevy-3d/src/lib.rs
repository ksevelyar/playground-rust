use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};

pub fn game_plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.05)))
        .insert_resource(GlobalAmbientLight {
            brightness: 0.0,
            ..default()
        })
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Playing), lock_cursor)
        .add_systems(OnEnter(GameState::Paused), (show_cursor, setup_pause_menu))
        .add_systems(
            Update,
            (
                toggle_pause,
                (mouse_look, move_player).run_if(in_state(GameState::Playing)),
            ),
        );
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}

const SENSITIVITY: f32 = 0.003;
const MOVE_SPEED: f32 = 5.0;
const PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.5, 5.0),
        CameraState::default(),
        Player,
    ));

    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            color: Color::srgb(1.0, 0.85, 0.6),
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 2.8, 0.0),
    ));

    // floor
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.0, 0.1, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.4, 0.4),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // walls
    let wall_mesh = meshes.add(Cuboid::new(10.0, 3.0, 0.1));
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.6),
        ..default()
    });

    let walls = [
        Transform::from_xyz(0.0, 1.5, -5.0),
        Transform::from_xyz(0.0, 1.5, 5.0),
        Transform::from_xyz(-5.0, 1.5, 0.0)
            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
        Transform::from_xyz(5.0, 1.5, 0.0)
            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
    ];

    for transform in walls {
        commands.spawn((
            Mesh3d(wall_mesh.clone()),
            MeshMaterial3d(wall_material.clone()),
            transform,
        ));
    }
}

pub fn toggle_pause(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(match state.get() {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        });
    }
}

fn lock_cursor(mut query: Query<&mut CursorOptions, With<Window>>) {
    let Ok(mut cursor_options) = query.single_mut() else {
        return;
    };
    cursor_options.grab_mode = CursorGrabMode::Locked;
    cursor_options.visible = false;
}

fn show_cursor(mut query: Query<&mut CursorOptions, With<Window>>) {
    let Ok(mut cursor_options) = query.single_mut() else {
        return;
    };
    cursor_options.grab_mode = CursorGrabMode::None;
    cursor_options.visible = true;
}

pub fn setup_pause_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Paused),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new("Press Esc to continue"),
            TextFont {
                font_size: FontSize::Px(48.0),
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    ));
}

pub fn mouse_look(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraState), With<Player>>,
) {
    let Ok((mut transform, mut state)) = query.single_mut() else {
        return;
    };

    let delta = mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    state.yaw -= delta.x * SENSITIVITY;
    state.pitch = (state.pitch - delta.y * SENSITIVITY).clamp(-PITCH_LIMIT, PITCH_LIMIT);

    transform.rotation = Quat::from_euler(EulerRot::YXZ, state.yaw, state.pitch, 0.0);
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CameraState), With<Player>>,
) {
    let Ok((mut transform, state)) = query.single_mut() else {
        return;
    };

    let forward = Vec3::new(-state.yaw.sin(), 0.0, -state.yaw.cos());
    let right = Vec3::new(state.yaw.cos(), 0.0, -state.yaw.sin());

    let mut direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        direction += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction -= forward;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction += right;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction -= right;
    }

    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * MOVE_SPEED * time.delta_secs();
    }
}
