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
        .init_resource::<GunshotSound>()
        .add_systems(Startup, (setup_room, setup_crosshair, setup_gun).chain())
        .add_systems(OnEnter(GameState::Playing), (lock_cursor, spawn_enemies, reset_player))
        .add_systems(OnExit(GameState::Playing), despawn_enemies)
        .add_systems(
            OnEnter(GameState::Paused),
            (show_cursor, setup_pause_menu),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            (show_cursor, setup_game_over_menu),
        )
        .add_systems(
            Update,
            (
                toggle_pause,
                restart_game.run_if(in_state(GameState::GameOver)),
                (
                    mouse_look,
                    move_player,
                    move_enemies_and_check_reach,
                    shoot,
                    play_gunshot,
                )
                    .run_if(in_state(GameState::Playing)),
                update_crosshair_and_gun_visibility,
            ),
        );
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Crosshair;

#[derive(Component)]
pub struct Gun;

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
    GameOver,
}

const MOUSE_SENSITIVITY: f32 = 0.003;
const PLAYER_MOVE_SPEED: f32 = 5.0;
const CAMERA_PITCH_LIMIT: f32 = std::f32::consts::FRAC_PI_2 - 0.01;

pub const ENEMY_RADIUS: f32 = 0.5;
const ENEMY_MOVE_SPEED: f32 = 1.0;
pub const ENEMY_KILL_DISTANCE: f32 = 0.8;

const GUN_BARREL_RADIUS: f32 = 0.03;
const GUN_BARREL_HEIGHT: f32 = 0.25;

pub const PLAYER_START_POSITION: Vec3 = Vec3::new(0.0, 1.5, 5.0);

pub fn setup_room(
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

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.0, 0.1, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.4, 0.4),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    let wall_mesh = meshes.add(Cuboid::new(10.0, 3.0, 0.1));
    let wall_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.6, 0.6),
        ..default()
    });

    let wall_transforms = [
        Transform::from_xyz(0.0, 1.5, -5.0),
        Transform::from_xyz(0.0, 1.5, 5.0),
        Transform::from_xyz(-5.0, 1.5, 0.0)
            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
        Transform::from_xyz(5.0, 1.5, 0.0)
            .with_rotation(Quat::from_rotation_y(90.0_f32.to_radians())),
    ];

    for wall_transform in wall_transforms {
        commands.spawn((
            Mesh3d(wall_mesh.clone()),
            MeshMaterial3d(wall_material.clone()),
            wall_transform,
        ));
    }
}

pub fn setup_crosshair(mut commands: Commands) {
    commands.spawn((
        Crosshair,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        children![
            (
                Node {
                    width: Val::Px(20.0),
                    height: Val::Px(2.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    margin: UiRect {
                        left: Val::Px(-10.0),
                        top: Val::Px(-1.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ),
            (
                Node {
                    width: Val::Px(2.0),
                    height: Val::Px(20.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(50.0),
                    margin: UiRect {
                        left: Val::Px(-1.0),
                        top: Val::Px(-10.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ),
        ],
    ));
}

pub fn setup_gun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<Entity, With<Player>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    let gun_barrel_mesh = meshes.add(Cylinder::new(GUN_BARREL_RADIUS, GUN_BARREL_HEIGHT));
    let gun_barrel_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.3, 0.3),
        ..default()
    });

    commands.entity(player_entity).with_children(|parent| {
        parent.spawn((
            Gun,
            Mesh3d(gun_barrel_mesh),
            MeshMaterial3d(gun_barrel_material),
            Transform::from_xyz(0.2, -0.2, -0.4)
                .with_rotation(Quat::from_rotation_x(-90.0_f32.to_radians())),
        ));
    });
}

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let enemy_mesh = meshes.add(Sphere::new(ENEMY_RADIUS));
    let enemy_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.1, 0.1),
        ..default()
    });

    let enemy_positions = [
        Vec3::new(-3.0, ENEMY_RADIUS, -3.0),
        Vec3::new(3.0, ENEMY_RADIUS, -3.0),
        Vec3::new(0.0, ENEMY_RADIUS, -4.0),
    ];

    for position in enemy_positions {
        commands.spawn((
            Enemy,
            Mesh3d(enemy_mesh.clone()),
            MeshMaterial3d(enemy_material.clone()),
            Transform::from_translation(position),
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn move_enemies_and_check_reach(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_position = player_transform.translation;

    for mut enemy_transform in enemy_query.iter_mut() {
        let distance = player_position.distance(enemy_transform.translation);
        if distance < ENEMY_KILL_DISTANCE {
            next_state.set(GameState::GameOver);
            return;
        }

        let direction_to_player = (player_position - enemy_transform.translation).normalize();
        enemy_transform.translation += direction_to_player * ENEMY_MOVE_SPEED * time.delta_secs();
    }
}

pub fn shoot(
    mouse_button: Res<ButtonInput<MouseButton>>,
    camera_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let camera_position = camera_transform.translation;
    let camera_forward = camera_transform.forward().as_vec3();

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let direction_to_enemy = enemy_transform.translation - camera_position;
        let distance_to_enemy = direction_to_enemy.length();
        let enemy_angular_radius = (ENEMY_RADIUS / distance_to_enemy).asin();
        let angle_to_enemy = camera_forward.dot(direction_to_enemy.normalize()).acos();

        if angle_to_enemy < enemy_angular_radius {
            commands.entity(enemy_entity).despawn();
            return;
        }
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
            GameState::GameOver => GameState::GameOver,
        });
    }
}

pub fn restart_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

pub fn reset_player(mut player_query: Query<(&mut Transform, &mut CameraState), With<Player>>) {
    let Ok((mut transform, mut camera_state)) = player_query.single_mut() else {
        return;
    };

    transform.translation = PLAYER_START_POSITION;
    camera_state.yaw = 0.0;
    camera_state.pitch = 0.0;
    transform.rotation = Quat::IDENTITY;
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

pub fn update_crosshair_and_gun_visibility(
    state: Res<State<GameState>>,
    mut crosshair_query: Query<&mut Visibility, With<Crosshair>>,
    mut gun_query: Query<&mut Visibility, (With<Gun>, Without<Crosshair>)>,
) {
    let should_be_visible = matches!(state.get(), GameState::Playing);

    if let Ok(mut crosshair_visibility) = crosshair_query.single_mut() {
        *crosshair_visibility = if should_be_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if let Ok(mut gun_visibility) = gun_query.single_mut() {
        *gun_visibility = if should_be_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
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

pub fn setup_game_over_menu(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::GameOver),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Text::new("skill issue!"),
                TextFont {
                    font_size: FontSize::Px(64.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.2, 0.2)),
            ),
            (
                Text::new("Press Space to restart"),
                TextFont {
                    font_size: FontSize::Px(32.0),
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ),
        ],
    ));
}

#[derive(Resource)]
pub struct GunshotSound {
    handle: Handle<AudioSource>,
}

impl FromWorld for GunshotSound {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        GunshotSound {
            handle: asset_server.load("pistol.mp3"),
        }
    }
}

pub fn play_gunshot(
    mouse_button: Res<ButtonInput<MouseButton>>,
    gunshot_sound: Res<GunshotSound>,
    mut commands: Commands,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        commands.spawn((
            AudioPlayer::new(gunshot_sound.handle.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}

pub fn mouse_look(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraState), With<Player>>,
) {
    let Ok((mut transform, mut camera_state)) = query.single_mut() else {
        return;
    };

    let mouse_delta = mouse_motion.delta;
    if mouse_delta == Vec2::ZERO {
        return;
    }

    camera_state.yaw -= mouse_delta.x * MOUSE_SENSITIVITY;
    camera_state.pitch = (camera_state.pitch - mouse_delta.y * MOUSE_SENSITIVITY)
        .clamp(-CAMERA_PITCH_LIMIT, CAMERA_PITCH_LIMIT);

    transform.rotation =
        Quat::from_euler(EulerRot::YXZ, camera_state.yaw, camera_state.pitch, 0.0);
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &CameraState), With<Player>>,
) {
    let Ok((mut transform, camera_state)) = query.single_mut() else {
        return;
    };

    let forward_direction = Vec3::new(-camera_state.yaw.sin(), 0.0, -camera_state.yaw.cos());
    let right_direction = Vec3::new(camera_state.yaw.cos(), 0.0, -camera_state.yaw.sin());

    let mut movement_direction = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        movement_direction += forward_direction;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement_direction -= forward_direction;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement_direction += right_direction;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement_direction -= right_direction;
    }

    if movement_direction != Vec3::ZERO {
        transform.translation +=
            movement_direction.normalize() * PLAYER_MOVE_SPEED * time.delta_secs();
    }
}
