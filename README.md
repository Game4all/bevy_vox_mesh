
<h1>
<code>bevy_vox_mesh</code>
</h1>

<a href="https://crates.io/crates/bevy_vox_mesh">
<img height="24" src="https://img.shields.io/crates/v/bevy_vox_mesh?style=for-the-badge"/>
</a>

A plugin for the bevy engine which allows loading magica voxel `.vox` files directly into usable meshes. This uses mesh vertex coloring.


## Bevy compatibility

| Bevy version | Plugin version |
| ------------ | -------------- |
| 0.5          | 0.1, 0.2       |
| 0.8          | 0.4            |


## Usage

![demo screenshot](https://raw.githubusercontent.com/Game4all/bevy_vox_mesh/master/assets/screenshot.PNG)

```rust

use bevy::prelude::*;
use bevy_vox_mesh::VoxMeshPlugin;
use std::f32::consts::PI;

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(VoxMeshPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stdmats: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_scale((0.01, 0.01, 0.01).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.load("chicken.vox"),
        material: stdmats.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    });
}



```

Take a look in the `examples/` directory for a complete working example.

## Acknowledgements

This asset loader is powered by the awesome [`block-mesh-rs`](https://github.com/bonsairobo/block-mesh-rs) crate.