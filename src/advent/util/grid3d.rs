use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use num_traits::PrimInt;
use std::fmt::Debug;

#[derive(Resource)]
struct Plane<T> where T: PrimInt+Debug {
    grid: Vec<Vec<Vec<T>>>,
}


impl<T: PrimInt+Debug+Sync+Send + 'static> Plane<T> {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        plane: Res<Plane<T>>,
    ) {
        commands.spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 2.0, 0.0),
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            // The default cascade config is designed to handle large scenes.
            // As this example has a much smaller world, we can tighten the shadow
            // bounds for better visual quality.
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 4.0,
                maximum_distance: 10.0,
                ..default()
            }
                .into(),
            ..default()
        });

        // circular base
        commands.spawn(PbrBundle {
            mesh: meshes.add(shape::Circle::new(50.0).into()),
            material: materials.add(Color::YELLOW_GREEN.into()),
            transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ..default()
        });
        // cube
        for x in 0..plane.grid.len() {
            for y in 0..plane.grid[0].len() {
                for z in 0..plane.grid[0][0].len() {
                    if plane.grid[x][y][z] == T::zero() {
                        continue;
                    }
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(
                            Color::rgb_u8(
                                ((plane.grid[x][y][z].to_u64().unwrap() * 7) % 256) as u8,
                                ((plane.grid[x][y][z].to_u64().unwrap() * 5) % 256) as u8,
                                ((plane.grid[x][y][z].to_u64().unwrap() * 3) % 256) as u8,
                            )
                                .into(),
                        ),
                        transform: Transform::from_xyz(x as f32, z as f32, y as f32),
                        ..default()
                    });
                }
            }
        }
        // camera
        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
                ..default()
            },
            PanOrbitCamera::default(),
        ));
    }
}

pub fn visualize<T: PrimInt+Debug+Sync+Send + 'static>(grid: &Vec<Vec<Vec<T>>>) {
    let mut plane = Plane{grid: grid.clone()};
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(plane)
        .add_systems(Startup, Plane::<usize>::setup)
        .run();

}