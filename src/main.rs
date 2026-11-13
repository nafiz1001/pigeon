use bevy::{color::palettes::css::SILVER, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, scene.spawn())
        .run();
}

fn scene() -> impl SceneList {
    bsn_list! {
        // 1. Camera3d
        Camera3d
        Transform::from_xyz(0., 7., 7.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y)
        --
        // 2. PointLight
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            shadow_depth_bias: 0.2,
        }
        Transform {
            translation: Vec3::new(8.0, 16.0, 8.0),
        }
        --
        // 3. Plane
        Mesh3d({ asset_value(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)) })
        MeshMaterial3d<StandardMaterial>(asset_value(Color::from(SILVER)))
        --
        // 4. Capsule
        Mesh3d({ asset_value(Capsule3d::default()) })
        MeshMaterial3d<StandardMaterial>(asset_value(Color::WHITE))
        Transform {
            translation: Vec3::new(0.0, 1.0, 0.0),
        }
    }
}
