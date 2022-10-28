use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<FoxMoveEvent>()
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(setup_scene_once_loaded)
        .add_system(move_fox)
        .add_system(update_fox_animation.after(setup_scene_once_loaded))
        .run();
}

// #[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Default)]
struct FoxMoveEvent;

#[derive(Component)]
struct Fox;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn()
    .insert_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 500., 1000.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.,
    });

    commands.spawn()
    .insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 1000.,
        })),
        material: materials.add(Color::DARK_GREEN.into()),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });

    commands.spawn()
    .insert_bundle(SceneBundle {
        scene: asset_server.load("models/Fox.glb#Scene0"),
        ..default()
    })
    .insert(Fox);

    commands.insert_resource(Animations(vec![
        asset_server.load("models/Fox.glb#Animation0"),
        asset_server.load("models/Fox.glb#Animation1"),
        asset_server.load("models/Fox.glb#Animation2"),
    ]));
}

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn move_fox(
    mut query: Query<&mut Transform, With<Fox>>,
    keyboard: Res<Input<KeyCode>>,
    mut events: EventWriter<FoxMoveEvent>,
) {
    let mut fox_transform = query.single_mut();
    let mut tmp = fox_transform.translation;

    if keyboard.pressed(KeyCode::H) {
        fox_transform.translation.x -= 3.;
        tmp.x += 100.;
        // fox_transform.look_at(tmp, Vec3::Y);
        events.send_default();
    }
    if keyboard.pressed(KeyCode::L) {
        fox_transform.translation.x += 3.;
        tmp.x -= 100.;
        // fox_transform.look_at(Vec3::new(-1000.,0.,0.), Vec3::Y);
        events.send_default();
    }
    if keyboard.pressed(KeyCode::J) {
        fox_transform.translation.z += 3.;
        tmp.z -= 100.;
        // fox_transform.look_at(Vec3::new(0.,0.,-1000.), Vec3::Y);
        events.send_default();
    }
    if keyboard.pressed(KeyCode::K) {
        fox_transform.translation.z -= 3.;
        tmp.z += 100.;
        // fox_transform.look_at(Vec3::new(0.,0.,1000.), Vec3::Y);
        events.send_default();
    }
    if keyboard.any_pressed([KeyCode::H, KeyCode::L, KeyCode::J, KeyCode::K]) {
        fox_transform.look_at(tmp, Vec3::Y);
    }
}

fn update_fox_animation(
    mut player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
    events: EventReader<FoxMoveEvent>,
) {
    if let Ok(mut player) = player.get_single_mut() {
    //     println!("{}", events.is_empty());
        if !events.is_empty() && *current_animation == 0 {
            *current_animation = 1;
            player.play(animations.0[*current_animation].clone_weak()).repeat();
        }
        if events.is_empty() && *current_animation == 1 {
            *current_animation = 0;
            player.play(animations.0[*current_animation].clone_weak()).repeat();
        }
    }
}