<h1>
<code>bevy_vox_scene</code>
</h1>

<a href="https://crates.io/crates/bevy_vox_mesh">
<img height="24" src="https://img.shields.io/crates/v/bevy_vox_mesh?style=for-the-badge"/>
</a>

A plugin for the bevy engine which allows loading magica voxel `.vox` files directly into usable meshes.
`bevy_vox_scene` is forked from the excellent [`bevy_vox_mesh`](https://crates.io/crates/bevy_vox_mesh).

## Why `bevy-vox-scene`?

`bevy_vox_scene` can load an entire scene graph from a Magica Voxel world file, and it attempts to recreate the material properties from Magica Voxel's render tab, so that you can produce a scene in Bevy that approximates Magica Voxel's raytraced renders, but at Bevy's real-time interactive framerates.

Here is the [study example scene](examples/transmission-scene.rs) as rendered by Magica Voxel's raytracer:
![The study scene rendered in Magica Voxel](assets/studyMV.jpg)

And this is the same scene in Bevy:
![The same scene in Bevy](assets/study-bevy.jpg)

All Magica Voxel material types except "cloud" are supported. Bevy's screen space transmission does a great job of rendering glass materials. 

`bevy_vox_scene` achieves this by generating a series of texture atlases for the scene to capture the differing color, metalness, roughness, emission, and transparency for each Voxel type in the scene.

## Usage

1. Add the crate using cargo: `cargo add bevy_vox_scene`
2. Import the library: 
```rust
use bevy_vox_scene::{VoxScenePlugin, VoxelSceneBundle};
```

3. Add the plugin to the app: 
```rust
app.add_plugins(VoxScenePlugin)
```

4. Spawn an entire scene graph using `VoxelSceneBundle`:
```rust
commands.spawn(VoxelSceneBundle {
    scene: assets.load("study.vox"),
    transform: Transform::IDENTITY,
});
```
Alternatively, spawn individual meshes using the name assigned to it in MagicaVoxel:

```rust
commands.spawn(PbrBundle {
    mesh: assets.load("study.vox#desk"),
    material: assets.load("study.vox#material_opaque"),
    ..default()
});
```

Take a look in the `examples/` directory for complete working examples. To run an example, enter:
```
cargo run --example <example name>
```

- If you want glowing emissive voxels, add an HDR and bloom-enabled camera. See the [`emissive-model` example](/examples/emissive-model.rs).
- Enabling Screen-Space Ambient Occlusion can give your voxel scenes more pop. See the [`ssao-model` example](/examples/ssao-model.rs).
- If you want glass voxels to refract other objects in the scene, enable specular transmission on your camera3d. See the [`transmission-scene` example](/examples/transmission-scene.rs).

## Bevy and Magica Voxel compatibility

| Bevy version | Magica Voxel version | `bevy-vox-scene` version | 
| ------------ | -------------- | --- |
| 0.12          | 0.99.6 | 0.9       | 

## Limitations and workarounds

- Positioning of models can be off if they have empty space around them. Fix this by tapping the "Fit model size" button in Magica Voxel.
- In MagicaVoxel's raytraced renders, emissive materials contribute to the lighting of a scene. Emissive materials do not currently do this in Bevy, they just glow. If in future Bevy implements a global illumination system, then emissive materials would contribute to the lighting.
- MagicaVoxel "cloud" materials are not currently supported.

### Transparent materials

TLDR: move glass voxels to a separate model in Magica Voxel's world editor (tip: you might need to manually move the trannsmissive models to last in Magica Voxel's render order for other models in the scene to be visible through them. Tap "Order -> Last" on the model that has the glass voxels)

- If you have a concave model that contains glass voxels, the other parts of that model will not be visible through the glass voxels. This is a limitation of Bevy's screen-space specular transmission system. To work around this limitation, use the Magica Voxel world editor to break up models that contain glass elements into separate models that are each convex.
- Bevy's StandardMaterial only allows a single Index of Refraction (IoR) per material. The IoR contained in a model are averaged together to arrive at this value. If your scene contains transmissive materials that have differing IoRs (eg glass and water), and if merging the IoRs for each medium makes a significant visible difference to the scene, consider breaking the model up into separate meshes, with a single IoR within each mesh.
- Bevy's Screen Space Ambient Occlusion (SSAO) appears to block the blurring affect that you get from glass materials that have roughness. If you have rough glass materials, consider not using SSAO.

## Differences between `bevy_vox_scene` and `bevy_vox_mesh`

|                     |`bevy_vox_mesh`|`bevy_vox_scene`|
| --- | --- | --- |
| __Voxel materials__ | Uses the vertex color attribute for voxel colors, doesn't attempt to display other voxel material properties | Generates a texture atlas for voxel colors, and also for emission, roughness, metalness, and transparency, allowing you to more closely replicate the look of a Magica Voxel render in bevy |
| __UV mapping__      | Uses "Minecraft-style" UV mapping, where a single texture can be tiled over each voxel in a mesh | Uses texture atlas UV mapping in order to display more voxel material properties, with no expectation that a texture will be tiled over each voxel |
| __World assets__    | Access different models contained in a single world file using sub-assets labelled by their index in the file `#model{index}`. This indexing system gets difficult to maintain with large world files, and can change if you delete models within the file. | Access different models using the name assigned to an instance of the model in the world file `#{my-named-model}`. User is responsible for ensuring the models you want to load have a name that is unique within that file. |

Which crate you should use depends on your use-case, and the look you're going for:

### Comparison of use cases

|                     |`bevy_vox_mesh`|__`bevy_vox_scene`__|
| --- | --- | --- |
| __Look__ | "Minecraft-style", with a texture tiled over each voxel. Can have a different palette per model. | Recreate the look of a MagicaVoxel render with material properties such as emission, roughness and metalness, consistent palette across scene |
| __File organisation__ | Have a single model, or a small number of models, in each file | Have multiple models in a single MagicaVoxel world file |

## Acknowledgements

Forked from the excellent [`bevy_vox_mesh`](https://crates.io/crates/bevy_vox_mesh) by Lucas A.

Like `bevy-vox-mesh`, `bevy-vox-scene` uses `dot-vox` to parse the vox files and the greedy mesher from [`block-mesh-rs`] (https://github.com/bonsairobo/block-mesh-rs) to create efficient meshes.

Ported to bevy 0.12.0 thanks to @baranyildirim.