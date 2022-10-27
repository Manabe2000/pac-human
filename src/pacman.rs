use bevy::{prelude::*, sprite::MaterialMesh2dBundle, sprite::collide_aabb::collide};
use rand::Rng;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system(move_pacman)
        .add_system(check_for_collisions)
        .add_system(bevy::window::close_on_esc)
        .add_system(play_collision_sound.after(check_for_collisions))
        .add_system(warp_pacman.after(move_pacman))
        .run();
}

#[derive(Component)]
struct Pacman;

#[derive(Component)]
struct Enemy;

#[derive(Default)]
struct CollisionEvent;

#[derive(Default)]
struct CollisionSound(Handle<AudioSource>);

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    // spawn camera
    // commands.spawn(Camera2dBundle::default());
    commands.spawn()
    .insert_bundle(Camera2dBundle::default());

    // spawn audio
    commands.insert_resource(CollisionSound(asset_server.load("sounds/breakout_collision.ogg")));

    // spawn pacman
    commands.spawn()
    .insert_bundle(
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::default().into()).into(),
                material: materials.add(ColorMaterial::from(Color::YELLOW)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(Vec3::new(50., 50., 0.)),
                ..default()
            })
    .insert(Pacman);

    for _i in 0..5 {
    let x: f32 = rng.gen_range(-300.0..300.0);
    let y: f32 = rng.gen_range(-300.0..300.0);
    let z: f32 = 0.0;
    let enemy_translation: Vec3 = Vec3::new(x, y, z);

        // spawn enemy
        commands.spawn()
        .insert_bundle(
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::from(Color::BLUE),
                        ..default()
                    },
                    transform: Transform {
                        translation: enemy_translation,
                        scale: Vec3::new(50., 50., 0.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Enemy);
    }
}

fn move_pacman(mut query: Query<&mut Transform, With<Pacman>>, keyboard_input: Res<Input<KeyCode>>,) {
    let mut pacman_transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::H) {
        pacman_transform.translation.x -= 5.;
    }
    if keyboard_input.pressed(KeyCode::L) {
        pacman_transform.translation.x += 5.;
    }
    if keyboard_input.pressed(KeyCode::K) {
        pacman_transform.translation.y += 5.;
    }
    if keyboard_input.pressed(KeyCode::J) {
        pacman_transform.translation.y -= 5.;
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut pacman_query: Query<&Transform, With<Pacman>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut collision_events: EventWriter<CollisionEvent>,
    ) {
    let pacman_transform = pacman_query.single_mut();
    for (enemy_entity, enemy_transform) in &enemy_query {

        let collision = collide(
            pacman_transform.translation,
            pacman_transform.scale.truncate(),
            enemy_transform.translation,
            enemy_transform.scale.truncate(),
            );

        if let Some(_collision) = collision {
            println!("collision!");
            collision_events.send_default();
            commands.entity(enemy_entity).despawn();
        }
    }
}

fn play_collision_sound(
    collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
    ){

    if !collision_events.is_empty() {
        audio.play(sound.0.clone());
    }
}

fn warp_pacman(
    mut windows: ResMut<Windows>,
    mut pacman_query: Query<&mut Transform, With<Pacman>>,
    ){

    let window = windows.primary_mut();
    let mut pacman_transform = pacman_query.single_mut();

    if pacman_transform.translation.x.abs() > window.width() / 2.{
        if pacman_transform.translation.x > 0. {
            pacman_transform.translation.x -= window.width();
        }
        else {
            pacman_transform.translation.x += window.width();
        }
    }
    if pacman_transform.translation.y.abs() > window.height() / 2.{
        if pacman_transform.translation.y > 0. {
            pacman_transform.translation.y -= window.height();
        }
        else {
            pacman_transform.translation.y += window.height();
        }
    }
}
