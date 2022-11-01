use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
    math::Vec3Swizzles,
};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<FoxMoveEvent>()
        .add_event::<FoxRunEvent>()
        .add_event::<CollisionEvent>()
        .insert_resource(ScoreBoard{ score: 0})
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_system(setup_scene_once_loaded)
        .add_system(run_fox)
        .add_system(move_fox.after(run_fox))
        .add_system(update_fox_animation.after(setup_scene_once_loaded))
        .add_system(check_for_collisions_with_fox)
        .add_system(respawn_cube.after(check_for_collisions_with_fox))
        // .add_system(update_camera_transform.after(move_fox))
        .add_system(update_scoreboard.after(check_for_collisions_with_fox))
        .run();
}

// #[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

struct ScoreBoard {
    score: u32
}

#[derive(Default)]
struct FoxMoveEvent;

#[derive(Default)]
struct FoxRunEvent;

#[derive(Default)]
struct CollisionEvent;

#[derive(Component)]
struct Fox;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Camera;

#[derive(Component)]
struct Size(Vec3);

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
    })
    .insert(Camera);

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 5.,
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
    .insert_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 100.})),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(100., 50., 100.),
        ..default()
    })
    .insert(Collider)
    .insert(Size(Vec3::splat(100.)));

    commands.spawn()
    .insert_bundle(SceneBundle {
        scene: asset_server.load("models/Fox.glb#Scene0"),
        ..default()
    })
    .insert(Fox)
    .insert(Size(Vec3::new(25., 70., 125.)));

    commands.insert_resource(Animations(vec![
        asset_server.load("models/Fox.glb#Animation0"),
        asset_server.load("models/Fox.glb#Animation1"),
        asset_server.load("models/Fox.glb#Animation2"),
    ]));

    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 50.,
        color: Color::WHITE,
    };
    commands.spawn()
    .insert_bundle(TextBundle::from_section(
        "Score: 0",
        text_style,
    ).with_style( Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Px(5.),
            left: Val::Px(5.),
            ..default()
        },
        ..default()
    }));
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

fn run_fox(
    keyboard: Res<Input<KeyCode>>,
    mut fox_run_events: EventWriter<FoxRunEvent>,
) {
    if keyboard.pressed(KeyCode::Space) {
        fox_run_events.send_default();
    }
}

fn move_fox(
    mut query: Query<&mut Transform, With<Fox>>,
    keyboard: Res<Input<KeyCode>>,
    mut fox_move_events: EventWriter<FoxMoveEvent>,
    fox_run_events: EventReader<FoxRunEvent>,
) {
    let mut fox_transform = query.single_mut();
    let mut tmp = fox_transform.translation;
    let fox_speed = if fox_run_events.is_empty() {3.0} else {10.0};

    if keyboard.pressed(KeyCode::H) {
        fox_transform.translation.x -= fox_speed;
        tmp.x += 100.;
    }
    if keyboard.pressed(KeyCode::L) {
        fox_transform.translation.x += fox_speed;
        tmp.x -= 100.;
    }
    if keyboard.pressed(KeyCode::J) {
        fox_transform.translation.z += fox_speed;
        tmp.z -= 100.;
    }
    if keyboard.pressed(KeyCode::K) {
        fox_transform.translation.z -= fox_speed;
        tmp.z += 100.;
    }
    if keyboard.any_pressed([KeyCode::H, KeyCode::L, KeyCode::J, KeyCode::K]) {
        if !(tmp == fox_transform.translation) {
            fox_transform.look_at(tmp, Vec3::Y);
            fox_move_events.send_default();
        }
    }
}

fn update_fox_animation(
    mut player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
    fox_move_events: EventReader<FoxMoveEvent>,
    fox_run_events: EventReader<FoxRunEvent>,
) {
    if let Ok(mut player) = player.get_single_mut() {
    //     println!("{}", events.is_empty());
        if fox_move_events.is_empty() && *current_animation != 0 {
            *current_animation = 0;
            player.play(animations.0[*current_animation].clone_weak()).set_elapsed(2.).repeat();
        }
        if !fox_move_events.is_empty() && fox_run_events.is_empty() && *current_animation != 1 {
            *current_animation = 1;
            player.play(animations.0[*current_animation].clone_weak()).repeat();
        }
        if !fox_run_events.is_empty() && !fox_move_events.is_empty() && *current_animation != 2 {
            *current_animation = 2;
            player.play(animations.0[*current_animation].clone_weak()).repeat();
            let speed = player.speed();
            player.set_speed(speed * 1.8);
        }
    }
}

fn check_for_collisions_with_fox(
    mut commands: Commands,
    fox_query: Query<(&Transform, &Size), With<Fox>>,
    collider_query: Query<(Entity, &Transform, &Size), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
    mut scoreboard: ResMut<ScoreBoard>,
) {
    let (fox_transform, fox_size) = fox_query.single();
    for (collider_entity, collider_transform, collider_size) in &collider_query {
        let fox_size = if fox_transform.forward().x == 0. {fox_size.0.xz()} else {fox_size.0.zx()};
        let collision = collide(
            fox_transform.translation.xzy(),
            fox_size,
            collider_transform.translation.xzy(),
            collider_size.0.xz(),
        );

        if let Some(collision) = collision {
            commands.entity(collider_entity).despawn();
            collision_events.send_default();
            scoreboard.score += 1;
        }
    }
}

fn respawn_cube(
    mut commands: Commands,
    collision_events: EventReader<CollisionEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !collision_events.is_empty() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-500.0..500.0);
        let y = 50.;
        let z = rng.gen_range(-500.0..500.0);
        let cube_translation = Vec3::new(x, y, z);

        commands.spawn()
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 100.})),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_translation(cube_translation),
            ..default()
        })
        .insert(Collider)
        .insert(Size(Vec3::splat(100.)));

        collision_events.clear();
    }
}

fn update_scoreboard(
    scoreboard: Res<ScoreBoard>,
    mut query: Query<&mut Text>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Score: {}", scoreboard.score.to_string());
}

// fn update_camera_transform(
//     mut query: Query<(&mut Transform, Option<&Camera>, Option<&Fox>)>,
// ) {
//     let mut camera_transform: Transform;
//     let mut fox_transform: Transform;
//     for (transform, maybe_camera, maybe_fox) in &query {
//         if maybe_camera.is_some() {
//             camera_transform = *transform;
//         }
//         if maybe_fox.is_some() {
//             fox_transform = *transform;
//         }
//     }

//     // camera_transform.translation.x = fox_transform.translation.x;
//     camera_transform.look_at(fox_transform.translation, Vec3::Y);
// }