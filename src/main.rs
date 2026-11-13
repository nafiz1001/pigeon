use bevy::{color::palettes::css::SILVER, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 7., 7.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    let debug_material = materials.add(StandardMaterial {
        base_color: Srgba::WHITE.into(),
        ..default()
    });

    let capsule = meshes.add(Capsule3d::default());

    commands.spawn((
        Mesh3d(capsule),
        MeshMaterial3d(debug_material),
        Transform::from_xyz(0., 1., 0.),
    ));
}
