use bevy::{color::palettes::css::SILVER, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, scene.spawn())
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn scene() -> impl SceneList {
    bsn_list! {
        #Camera
        Camera3d
        Transform::from_xyz(0., 7., 7.).looking_at(Vec3::new(0., 1., 0.), Vec3::Y)
        --
        #Light
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
        #Ground
        Mesh3d({ asset_value(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)) })
        MeshMaterial3d<StandardMaterial>(asset_value(Color::from(SILVER)))
        --
        #Capsule
        Mesh3d({ asset_value(Capsule3d::default()) })
        MeshMaterial3d<StandardMaterial>(asset_value(Color::WHITE))
        Transform {
            translation: Vec3::new(0.0, 1.0, 0.0),
            rotation: Quat::from_rotation_x(90.0_f32.to_radians())
        }
    }
}
