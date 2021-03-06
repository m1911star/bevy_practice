use bevy::prelude::*;

pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(1.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 100.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.2, 0.2),
            perceptual_roughness: 0.08,
            ..Default::default()
        }),
        ..Default::default()
    });
    const COUNT: usize = 6;
    let postion_range = -4.0..4.0;
    let radius_range = 0.0..0.8;
    // let pos_len = postion_range.end - postion_range.start;
    let radius_len = radius_range.end - radius_range.start;
    let mesh = meshes.add(Mesh::from(shape::UVSphere {
        sectors: 128,
        stacks: 64,
        ..Default::default()
    }));

    for i in 0..COUNT {
        let percent = i as f32 / COUNT as f32;
        let radius = radius_range.start + percent * radius_len;

        commands
            .spawn_bundle(PbrBundle {
                mesh: mesh.clone(),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.5, 0.5, 1.0),
                    unlit: true,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .with_children(|children| {
                children.spawn_bundle(PointLightBundle {
                    point_light: PointLight {
                        intensity: 1500.0,
                        radius,
                        color: Color::rgb(0.2, 0.2, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
    }
}
