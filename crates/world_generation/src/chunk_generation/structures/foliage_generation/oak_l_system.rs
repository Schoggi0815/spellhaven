use bevy::math::{FloatExt, Vec3};
use rand::{Rng, rngs::StdRng};

use crate::chunk_generation::{
    VOXEL_SIZE,
    block_type::BlockType,
    structures::foliage_generation::{
        oak_options::OakOptions,
        tree_l_system::{LSystem, LSystemEntry},
    },
};

pub struct OakLSystem;

#[derive(Clone, Copy)]
pub enum OakEntryType {
    Stem,
    Branch { angle_x: f32, angle_z: f32 },
    Leaf,
}

impl LSystem<OakEntryType, OakOptions> for OakLSystem {
    fn get_start_state(
        position: Vec3,
        rng: &mut StdRng,
        grow_options: &OakOptions,
    ) -> Vec<LSystemEntry<OakEntryType>> {
        Self::create_straight_piece(
            &position,
            grow_options.start_x_angle,
            grow_options.start_y_angle,
            grow_options.start_thickness,
            (rng.random_range(4.5..6.5) / VOXEL_SIZE) as usize,
            OakEntryType::Stem,
            OakEntryType::Branch {
                angle_x: grow_options.start_x_angle,
                angle_z: grow_options.start_y_angle,
            },
        )
    }

    fn process_tree(
        mut start_state: &mut Vec<LSystemEntry<OakEntryType>>,
        rng: &mut StdRng,
        grow_options: &OakOptions,
    ) {
        for _ in 0..6 {
            Self::recurse_l_system(&mut start_state, rng, grow_options);
        }
        Self::add_leafs(&mut start_state);
    }

    fn get_block_from_entry(entry: &LSystemEntry<OakEntryType>) -> BlockType {
        match entry.entry_type {
            OakEntryType::Leaf => BlockType::Leaf,
            _ => BlockType::Log,
        }
    }

    fn recurse_entry(
        entry: &LSystemEntry<OakEntryType>,
        rng: &mut StdRng,
        branches: &mut Vec<LSystemEntry<OakEntryType>>,
        grow_options: &OakOptions,
    ) {
        if let OakEntryType::Branch { angle_x, angle_z } = entry.entry_type {
            let mut max_thickness = entry.thickness;

            while max_thickness >= grow_options.min_thickness {
                let lerp_value = rng.random_range::<f32, _>(0.0..1.0).powf(0.5);
                let thickness =
                    grow_options.min_thickness.lerp(max_thickness, lerp_value);
                max_thickness -= thickness * 0.3;

                let max_angle_adjusted =
                    grow_options.max_angle * (1. - lerp_value);

                let angle_x_lerp =
                    rng.random_range::<f32, _>(0.0..1.0).powf(0.5);
                let angle_x_lerp =
                    angle_x_lerp.copysign(rng.random_range(-1.0..1.0));
                let angle_z_lerp =
                    rng.random_range::<f32, _>(0.0..1.0).powf(0.5);
                let angle_z_lerp =
                    angle_z_lerp.copysign(rng.random_range(-1.0..1.0));

                let angle_x = angle_x_lerp * max_angle_adjusted + angle_x;
                let angle_z = angle_z_lerp * max_angle_adjusted + angle_z;

                let length = grow_options
                    .min_length
                    .lerp(grow_options.max_length, lerp_value)
                    as usize;

                branches.extend(Self::create_straight_piece(
                    &entry.pos,
                    angle_x
                        .min(grow_options.max_angle)
                        .max(-grow_options.max_angle),
                    angle_z
                        .min(grow_options.max_angle)
                        .max(-grow_options.max_angle),
                    thickness,
                    length,
                    OakEntryType::Stem,
                    OakEntryType::Branch { angle_x, angle_z },
                ));
            }
        }
    }
}

impl OakLSystem {
    fn add_leafs(data: &mut Vec<LSystemEntry<OakEntryType>>) {
        let mut i = 0usize;
        while i < data.len() {
            let entry = &data[i];
            if let OakEntryType::Branch { .. } = entry.entry_type {
                let branches = vec![LSystemEntry {
                    pos: entry.pos,
                    entry_type: OakEntryType::Leaf,
                    thickness: 2.0,
                }];

                let length = branches.len();

                data.splice(i..i + 1, branches);
                i += length;
            }
            i += 1;
        }
    }
}
