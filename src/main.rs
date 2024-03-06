use std::{f32::consts::PI, time::Duration};

use bevy::{
    animation::{AnimationClip, AnimationPlayer, RepeatAnimation}, app::{App, Startup, Update}, asset::{AssetServer, Assets, Handle}, core_pipeline::core_3d::Camera3dBundle, ecs::{
        query::Added,
        system::{Commands, Local, Query, Res, ResMut, Resource},
    }, input::{keyboard::KeyCode, ButtonInput}, math::{primitives::Plane3d, EulerRot, Quat, Vec3}, pbr::{
        AmbientLight, CascadeShadowConfigBuilder, DirectionalLight, DirectionalLightBundle,
        PbrBundle, StandardMaterial,
    }, render::{
        color::Color,
        mesh::{Mesh, Meshable},
    }, scene::SceneBundle, transform::components::Transform, utils::default, DefaultPlugins
};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (setup_scene_once_loaded,keyboard_animation_control))
        .run()
}
#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Animations(vec![
        asset_server.load("Fox.glb#Animation2"),
        asset_server.load("Fox.glb#Animation1"),
        asset_server.load("Fox.glb#Animation0"),
        // asset_server.load("Fox.glb#Animation3"),
        // asset_server.load("Fox.glb#Animation4"),
        // asset_server.load("Fox.glb#Animation5"),
        // asset_server.load("Fox.glb#Animation6"),
        // asset_server.load("Fox.glb#Animation7"),
        // asset_server.load("Fox.glb#Animation8"),
        // asset_server.load("Fox.glb#Animation9"),
    ]));
    //camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(100.0, 100.0, 150.0)
            .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
        ..default()
    });

    //Light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, 5.0, -PI / 4.)),
        directional_light: DirectionalLight {
            color:Color::rgb(0.2, 0.6, 0.4),
            shadows_enabled: true,
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .into(),
        ..default()
    });
    //Fox
    commands.spawn(SceneBundle {
        scene: asset_server.load("Fox.glb#Scene0"),
        transform:Transform::from_scale(Vec3::new(30.0    , 30.0, 30.0)),
        ..default()
    });
    //plan
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0)),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3)),
        ..default()
    });
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[2].clone_weak()).repeat();
    }
}


fn keyboard_animation_control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players:Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation:Local<usize>
) {
    for mut player in &mut animation_players {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            let elapsed = player.seek_time();
            player.seek_to(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Enter) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play_with_transition(
                    animations.0[*current_animation].clone_weak(),
                    Duration::from_millis(250),
                )
                .repeat();
        }

        if keyboard_input.just_pressed(KeyCode::Digit1) {
            player.set_repeat(RepeatAnimation::Count(1));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Digit3) {
            player.set_repeat(RepeatAnimation::Count(3));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::Digit5) {
            player.set_repeat(RepeatAnimation::Count(5));
            player.replay();
        }

        if keyboard_input.just_pressed(KeyCode::KeyL) {
            player.set_repeat(RepeatAnimation::Forever);
        }
    }
}