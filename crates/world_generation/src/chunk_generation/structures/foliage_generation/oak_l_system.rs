use std::ops::Range;

use bevy::{
    log::info,
    math::{FloatExt, Vec3},
};
use rand::{Rng, rngs::StdRng};

use crate::chunk_generation::{
    VOXEL_SIZE,
    block_type::BlockType,
    structures::foliage_generation::tree_l_system::{LSystem, LSystemEntry},
};

pub struct OakLSystem;

#[derive(Clone, Copy)]
pub enum OakEntryType {
    Stem,
    Branch { angle_x: f32, angle_z: f32 },
    Leaf,
}

impl LSystem<OakEntryType> for OakLSystem {
    fn get_start_state(
        position: Vec3,
        rng: &mut StdRng,
    ) -> Vec<LSystemEntry<OakEntryType>> {
        let x_angle = rng.random_range(-10.0..10.0);
        let y_angle = rng.random_range(-10.0..10.0);

        Self::create_straight_piece(
            &position,
            x_angle,
            y_angle,
            2.0,
            (rng.random_range(4.5..6.5) / VOXEL_SIZE) as usize,
            OakEntryType::Stem,
            OakEntryType::Branch {
                angle_x: x_angle,
                angle_z: y_angle,
            },
        )
    }

    fn process_tree(
        mut start_state: &mut Vec<LSystemEntry<OakEntryType>>,
        rng: &mut StdRng,
    ) {
        for _ in 0..6 {
            Self::recurse_l_system(&mut start_state, rng);
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
    ) {
        if let OakEntryType::Branch { angle_x, angle_z } = entry.entry_type {
            let mut max_thickness = entry.thickness;
            let min_thickness = 0.6;

            let max_length = 7.;
            let min_length = 4.;

            let max_angle = 60.;

            while max_thickness >= min_thickness {
                let lerp_value = rng.random_range::<f32, _>(0.0..1.0).powf(0.5);
                let thickness = min_thickness.lerp(max_thickness, lerp_value);
                max_thickness -= thickness * 0.3;

                let max_angle_adjusted = max_angle * (1. - lerp_value);

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

                let length = min_length.lerp(max_length, lerp_value) as usize;

                branches.extend(Self::create_straight_piece(
                    &entry.pos,
                    angle_x.min(max_angle).max(-max_angle),
                    angle_z.min(max_angle).max(-max_angle),
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
