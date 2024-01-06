<h1>
<code>bevy_vox_scene</code>
</h1>

<a href="https://crates.io/crates/bevy_vox_mesh">
<img height="24" src="https://img.shields.io/crates/v/bevy_vox_mesh?style=for-the-badge"/>
</a>

A plugin for the bevy engine which allows loading magica voxel `.vox` files directly into usable meshes.
`bevy_vox_scene` is forked from the excellent [`bevy_vox_mesh`](https://crates.io/crates/bevy_vox_mesh)

## Differences between `bevy_vox_scene` and `bevy_vox_mesh`

|                     |`bevy_vox_mesh`|`bevy_vox_scene`|
| --- | --- | --- |
| __Voxel materials__ | Uses the vertex color attribute for voxel colors, doesn't attempt to display other voxel material properties | Generates a texture atlas for voxel colors, and also for emission, roughness, and metalness, allowing you to more closely replicate the look of a Magica Voxel render in bevy |
| __UV mapping__      | Uses "Minecraft-style" UV mapping, where a single texture can be tiled over each voxel in a mesh | Uses texture atlas UV mapping in order to display more voxel material properties, with no expectation that a texture will be tiled over each voxel |
| __World assets__    | Access different models contained in a single world file using sub-assets labelled by their index in the file `#model{index}`. This indexing system gets difficult to maintain with large world files, and can change if you delete models within the file. | Access different models using the name assigned to an instance of the model in the world file `#{my-named-model}`. User is responsible for ensuring the models you want to load have a name that is unique within that file. |

Which crate you should use depends on your use-case, and the look you're going for:

### Comparison of use cases

|                     |`bevy_vox_mesh`|`bevy_vox_scene`|
| --- | --- | --- |
| __Look__ | "Minecraft-style", with a texture tiled over each voxel | Recreate the look of a MagicaVoxel render with material properties such as emission, roughness and metalness |
| __File organisation__ | Have a single model, or a small number of models, in each file, and don't need a consistent palette between them | Have multiple models in a single MagicaVoxel world file, with the same palette shared between them |


## Bevy compatibility

| Bevy version | `bevy-vox-mesh` version |
| ------------ | -------------- |
| 0.5          | 0.1, 0.2       |
| 0.8          | 0.4            |
| 0.9          | 0.5            |
| 0.10         | 0.6            |
| 0.11         | 0.7, 0.7.1     |
| 0.12         | 0.8            |

| Bevy version | `bevy-vox-scene` version | Magica Voxel version |
| ------------ | -------------- | --- |
| 0.12          | 0.9       | 0.99.6 |

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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        transform: Transform::from_scale((0.01, 0.01, 0.01).into())
            * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.load("chicken.vox"),
        material: stdmats.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    });
}



```

Take a look in the `examples/` directory for a complete working example.

## Limitations

- Positioning of models can be off if they have empty space around them. Fix this by tapping the "Fit model size" button in Magica Voxel.
- If you have a concave model that contains glass voxels, the other parts of that model will not be visible through the glass voxels. This is a limitation of Bevy's screen-space specular transmission system. To work around this limitation, use the Magica Voxel world editor to break up models that contain glass elements into separate models that are each convex.

## Acknowledgements

This asset loader is powered by the awesome [`block-mesh-rs`](https://github.com/bonsairobo/block-mesh-rs) crate.

Ported to bevy 0.12.0 thanks to @baranyildirim.