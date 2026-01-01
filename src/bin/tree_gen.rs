use bevy::{
    camera::Exposure,
    core_pipeline::tonemapping::Tonemapping,
    light::{SunDisk, light_consts::lux},
    pbr::{
        Atmosphere, ExtendedMaterial,
        wireframe::{WireframeConfig, WireframePlugin},
    },
    post_process::bloom::Bloom,
    prelude::*,
    render::view::{ColorGrading, ColorGradingGlobal},
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use rand::{Rng, SeedableRng, rng, rngs::StdRng};
use world_generation::{
    chunk_generation::{
        CHUNK_SIZE, VOXEL_SIZE,
        block_type::BlockType,
        chunk_lod::ChunkLod,
        mesh_generation::generate_mesh,
        noise::{
            terrain_noise::TerrainNoise,
            terrain_noise_type::{ConstantValue, TerrainNoiseType},
        },
        structures::{
            oak_structure_generator::OakStructureGenerator,
            pine_structure_generator::PineStructureGenerator,
            structure_generator::{StructureGenerator, VoxelStructureMetadata},
            tree_structure_generator::TreeStructureGenerator,
        },
        voxel_data::VoxelData,
    },
    terrain_material::TerrainMaterial,
};

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Spellhaven".into(),
                        ..default()
                    }),
                    ..default()
                }),
                PanOrbitCameraPlugin,
                WireframePlugin { ..default() },
                //BirdCameraPlugin,
                EguiPlugin::default(),
                WorldInspectorPlugin::new(),
                MaterialPlugin::<
                    ExtendedMaterial<StandardMaterial, TerrainMaterial>,
                >::default(),
            ),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, rebuild_tree_system)
        .insert_resource(WireframeConfig {
            global: false,
            default_color: Color::srgb(1., 0., 0.),
        })
        .run();
}

#[derive(Component)]
struct TreeGen;

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, TerrainMaterial>>,
    >,
) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: lux::RAW_SUNLIGHT,
            ..default()
        },
        SunDisk::EARTH,
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 3.),
            ..default()
        },
        Name::new("Light"),
    ));

    commands.spawn((
        Camera3d::default(),
        Camera {
            ..Default::default()
        },
        ColorGrading {
            global: ColorGradingGlobal {
                post_saturation: 1.2,
                ..Default::default()
            },
            ..Default::default()
        },
        Msaa::Sample2,
        Transform::from_xyz(-4.0, 6.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        Projection::Perspective(PerspectiveProjection {
            far: 2f32.powi(20),
            ..default()
        }),
        Exposure::SUNLIGHT,
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
        PanOrbitCamera::default(),
        Atmosphere::EARTH,
        Name::new("CAMMIE"),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: lux::FULL_DAYLIGHT,
        ..default()
    });

    spawn_mesh(commands, meshes, materials);
}

fn spawn_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, TerrainMaterial>>,
    >,
) {
    let chunks = get_tree_voxel_data();

    for (chunk, chunk_pos) in chunks {
        let mesh = generate_mesh(&chunk, ChunkLod::Full);

        let Some(mesh) = mesh.opaque_mesh else {
            continue;
        };

        commands.spawn((
            Transform::from_translation(
                chunk_pos.as_vec3() * CHUNK_SIZE as f32 * VOXEL_SIZE,
            ),
            Name::new("Chunk"),
            Mesh3d(meshes.add(mesh)),
            MeshMaterial3d(materials.add(ExtendedMaterial {
                base: StandardMaterial {
                    perceptual_roughness: 1.,
                    ..Default::default()
                },
                extension: TerrainMaterial {
                    chunk_position: chunk_pos.as_vec3(),
                    lod_multiplier: ChunkLod::Full.multiplier_i32() as u32,
                },
            })),
            TreeGen,
        ));
    }
}

fn rebuild_tree_system(
    mut tree_entities: Query<Entity, With<TreeGen>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<
        Assets<ExtendedMaterial<StandardMaterial, TerrainMaterial>>,
    >,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if !input.pressed(KeyCode::Space) {
        return;
    }

    for entity in &mut tree_entities {
        commands.entity(entity).despawn();
    }

    spawn_mesh(commands, meshes, materials);
}

fn get_tree_voxel_data() -> Vec<(Box<VoxelData>, IVec3)> {
    let mut chunks = vec![
        (Box::new(VoxelData::default()), IVec3::new(0, 0, 0)),
        (Box::new(VoxelData::default()), IVec3::new(0, 0, 1)),
        (Box::new(VoxelData::default()), IVec3::new(0, 1, 0)),
        (Box::new(VoxelData::default()), IVec3::new(0, 1, 1)),
        (Box::new(VoxelData::default()), IVec3::new(1, 0, 0)),
        (Box::new(VoxelData::default()), IVec3::new(1, 0, 1)),
        (Box::new(VoxelData::default()), IVec3::new(1, 1, 0)),
        (Box::new(VoxelData::default()), IVec3::new(1, 1, 1)),
        (Box::new(VoxelData::default()), IVec3::new(0, 2, 0)),
        (Box::new(VoxelData::default()), IVec3::new(0, 2, 1)),
        (Box::new(VoxelData::default()), IVec3::new(1, 2, 0)),
        (Box::new(VoxelData::default()), IVec3::new(1, 2, 1)),
    ];

    let seed = rng().random();

    let tree_generator = PineStructureGenerator::new(
        VoxelStructureMetadata::new(
            [45, 45, 45],
            [0, 0],
            [0, 0],
            get_tree_noise(),
            seed,
        ),
        &mut StdRng::seed_from_u64(seed),
    );

    let tree_model =
        tree_generator.get_structure_model(IVec2::new(0, 0), ChunkLod::Full);

    for (chunk, chunk_pos) in &mut chunks {
        apply_trees(chunk, *chunk_pos, &tree_model);
    }

    chunks
}

fn get_tree_noise() -> TerrainNoise {
    TerrainNoise::new(
        0,
        vec![
            TerrainNoiseType::Constant { value_index: 1 },
            TerrainNoiseType::RandomF64 {
                min_index: 2,
                max_index: 3,
            },
            TerrainNoiseType::ConstantValue {
                value: ConstantValue::F64(-1.),
            },
            TerrainNoiseType::ConstantValue {
                value: ConstantValue::F64(1.),
            },
        ],
    )
}

fn apply_trees(
    blocks: &mut VoxelData,
    chunk_position: IVec3,
    tree_model: &Vec<Vec<Vec<BlockType>>>,
) {
    let chunk_x = chunk_position.x * CHUNK_SIZE as i32;
    let chunk_y = chunk_position.y * CHUNK_SIZE as i32;
    let chunk_z = chunk_position.z * CHUNK_SIZE as i32;

    for x in chunk_x - 1..chunk_x + CHUNK_SIZE as i32 + 1 {
        if x < 0 {
            continue;
        }

        let x = x as usize;

        if x >= tree_model.len() {
            break;
        }

        let tree_model_x = &tree_model[x];
        for y in chunk_y - 1..chunk_y + CHUNK_SIZE as i32 + 1 {
            if y < 0 {
                continue;
            }

            let y = y as usize;

            if y >= tree_model_x.len() {
                break;
            }

            let tree_model_y = &tree_model_x[y];
            for z in chunk_z - 1..chunk_z + CHUNK_SIZE as i32 + 1 {
                if z < 0 {
                    continue;
                }

                let z = z as usize;

                if z >= tree_model_y.len() {
                    break;
                }

                let tree_model_block = tree_model_y[z];

                let chunk_x = x as i32 - chunk_x;
                let chunk_y = y as i32 - chunk_y;
                let chunk_z = z as i32 - chunk_z;

                blocks.set_block(
                    IVec3::new(chunk_x + 1, chunk_y + 1, chunk_z + 1),
                    tree_model_block,
                );
            }
        }
    }
}
