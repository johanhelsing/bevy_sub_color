# bevy_sub_color

Simple subtractive color material for drawing colored meshes with subtractive color blending.

![screenshot of basic example](https://johanhelsing.studio/assets/bevy_sub_color.png)

It just wraps Bevy's `ColorMaterial`, but changes the blend mode.

## Usage

```rust
.add_plugin(SubColorMaterialPlugin)
```

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // Just use SubColorMaterial instead of ColorMaterial
    mut materials: ResMut<Assets<SubColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Quad::default().into()),
        transform: Transform::from_scale(Vec3::splat(100.)),
        material: materials.add(Color::PINK.into()),
        ..Default::default()
    });
}
```

## License

All code in this repository dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.