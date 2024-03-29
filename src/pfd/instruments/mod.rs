use bevy::{
    prelude::*, 
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};

pub struct InstrumentsPlugin;

impl Plugin for InstrumentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, draw_circle));
    }
}

#[derive(Component)]
struct InstrumentsCameraMarker;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
            camera:  Camera {
                order: 10,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        }, 
        InstrumentsCameraMarker
    ));
}


#[derive(Component)]
struct HSI;

fn draw_circle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(190.0))),
        material: materials.add(Color::NONE.with_a(0.3)),
        transform: Transform {
            translation: Vec3::new(0.0, -140.0, 0.0),
            ..default()
        },
        ..default()
    },
    HSI
    ));
    

}
