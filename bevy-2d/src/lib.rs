use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, move_player);
}

#[derive(Component)]
pub struct Player;

const MOVEMENT_SPEED: f32 = 200.0;
const ROTATION_SPEED: f32 = 3.0;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new("@"),
        TextFont::from_font_size(64.0),
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
        Transform::default(),
        Player,
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = player_query.single_mut() else {
        return;
    };

    let delta_time = time.delta_secs();

    if keyboard_input.pressed(KeyCode::KeyA) {
        transform.rotate_z(ROTATION_SPEED * delta_time);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        transform.rotate_z(-ROTATION_SPEED * delta_time);
    }

    let forward_direction = transform.rotation * Vec3::Y;

    if keyboard_input.pressed(KeyCode::KeyW) {
        transform.translation += forward_direction * MOVEMENT_SPEED * delta_time;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        transform.translation -= forward_direction * MOVEMENT_SPEED * delta_time;
    }
}
