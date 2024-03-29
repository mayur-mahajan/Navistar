use bevy::prelude::*;
use std::f32::consts::PI;

use super::model::{Roll, Pitch};

const SKY_COLOR: Color = Color::rgb(77./255., 130./255., 247./255.);
const GROUND_COLOR: Color = Color::rgb(144./255., 102./255., 47./255.);

pub struct HorizonPlugin;

impl Plugin for HorizonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(SKY_COLOR));
        app.add_systems(Startup, (setup_camera, setup_light, setup_ground));
        app.add_systems(Update, (test_bank, update));
    }
}

#[derive(Component)]
struct MyCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 20.0)
                            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    }, 
    MyCameraMarker
    ));
}

fn setup_light(
    mut ambient_light: ResMut<AmbientLight>
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 300.0;
}

fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(500.0, 500.0)),
        material: materials.add(GROUND_COLOR),
        ..Default::default()
    },
    Name::new("Ground")
    ));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::from_size(Vec3::splat(0.2))),
        material: materials.add(Color::WHITE),
        ..default()
    },
    Name::new("Origin Marker")
    ));
}

fn update(
    roll: Res<Roll>,
    pitch: Res<Pitch>,
    mut camera_transform: Query<&mut Transform, With<MyCameraMarker>>
) {
    let mut camera_transform = camera_transform.single_mut();

    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
    camera_transform.rotate_z(roll.0);
    camera_transform.rotate_x(pitch.0);
}

fn test_bank(
    keys: Res<ButtonInput<KeyCode>>,
    mut roll: ResMut<Roll>,
    mut pitch: ResMut<Pitch>
) {

    if keys.pressed(KeyCode::KeyA) {
        debug!("Bank Left");
        roll.0 += PI / 720.0;
    }
    
    if keys.pressed(KeyCode::KeyD) {
        debug!("Bank Right");
        roll.0 -= PI / 720.0;
    }

    if keys.pressed(KeyCode::KeyW) {
        debug!("Pitch Up");
        pitch.0 += PI / 720.0;
    }

    if keys.pressed(KeyCode::KeyX) {
        debug!("Pitch Down");
        pitch.0 -= PI / 720.0;
    }

    if keys.just_pressed(KeyCode::KeyR) {
        pitch.0 = 0.0;
        roll.0 = 0.0;
    }
}