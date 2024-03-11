#[cfg(target_arch ="wasm32")]
#[wasm_bindgen]
fn run() {
    use bevy::{app::{App, Startup, Update}, pbr::AmbientLight, render::color::Color, DefaultPlugins};

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