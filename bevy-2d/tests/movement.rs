use bevy::prelude::*;
use bevy_2d::{game_plugin, Player};
use std::time::Duration;

fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(game_plugin);
    app.insert_resource(ButtonInput::<KeyCode>::default());
    app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
        Duration::from_millis(16),
    ));
    app
}

#[test]
fn setup_spawns_player() {
    let mut app = create_test_app();
    app.update();
    app.update();

    let mut player_query = app.world_mut().query::<&Player>();
    assert_eq!(player_query.iter(app.world()).count(), 1);
}

#[test]
fn w_moves_player_forward() {
    let mut app = create_test_app();
    app.update();

    let player_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world())
        .unwrap();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);

    app.update();
    app.update();

    let transform = app.world().get::<Transform>(player_entity).unwrap();
    assert!(transform.translation.y > 0.0);
}

#[test]
fn s_moves_player_backward() {
    let mut app = create_test_app();
    app.update();

    let player_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world())
        .unwrap();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyS);

    app.update();
    app.update();

    let transform = app.world().get::<Transform>(player_entity).unwrap();
    assert!(transform.translation.y < 0.0);
}

#[test]
fn a_rotates_counterclockwise() {
    let mut app = create_test_app();
    app.update();

    let player_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world())
        .unwrap();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyA);

    app.update();
    app.update();

    let transform = app.world().get::<Transform>(player_entity).unwrap();
    let (_, _, rotation_z) = transform.rotation.to_euler(EulerRot::XYZ);
    assert!(rotation_z > 0.0);
}

#[test]
fn d_rotates_clockwise() {
    let mut app = create_test_app();
    app.update();

    let player_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world())
        .unwrap();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyD);

    app.update();
    app.update();

    let transform = app.world().get::<Transform>(player_entity).unwrap();
    let (_, _, rotation_z) = transform.rotation.to_euler(EulerRot::XYZ);
    assert!(rotation_z < 0.0);
}

#[test]
fn forward_direction_changes_with_rotation() {
    let mut app = create_test_app();
    app.update();

    let player_entity = app
        .world_mut()
        .query_filtered::<Entity, With<Player>>()
        .single(app.world())
        .unwrap();

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyA);
    app.update();
    app.update();
    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .release(KeyCode::KeyA);

    let initial_x = app
        .world()
        .get::<Transform>(player_entity)
        .unwrap()
        .translation
        .x;

    app.world_mut()
        .resource_mut::<ButtonInput<KeyCode>>()
        .press(KeyCode::KeyW);
    app.update();
    app.update();

    let transform = app.world().get::<Transform>(player_entity).unwrap();
    assert!(
        transform.translation.x < initial_x,
        "After rotating 90 degrees CCW, forward should be -X"
    );
}
