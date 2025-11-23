use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};
use rand::{Rng, rngs::StdRng};
use utils::rotation::{RotationDirection, rotate_around};

use crate::chunk_generation::{
    VOXEL_SIZE,
    block_type::BlockType,
    structures::foliage_generation::{
        entry_range::EntryRange,
        pine_options::PineOptions,
        tree_l_system::{LSystem, LSystemEntry},
    },
};

pub struct PineLSystem;

#[derive(Clone, Copy)]
pub enum PineEntryType {
    Log,
    Stem {
        branch_length: f32,
        branch_thickness: EntryRange,
    },
    Branch {
        direction: Vec3,
        tip: bool,
        sub_length: f32,
    },
    SubBranch {
        direction: Vec3,
        tip: bool,
    },
    Needle,
}

impl LSystem<PineEntryType, PineOptions> for PineLSystem {
    fn get_start_state(
        position: Vec3,
        rng: &mut StdRng,
        _: &PineOptions,
    ) -> Vec<LSystemEntry<PineEntryType>> {
        let mut entries = vec![];

        let length = (rng.random_range(4.0..6.0) / VOXEL_SIZE) as usize;
        let mut last_length = length;
        let total_thickness_range: EntryRange = (3.0..0.8).into();
        let stem_count = 12;
        let total_branch_thickness_range: EntryRange = (1.5..0.5).into();

        entries.extend(Self::create_straight_piece_dir(
            position,
            Vec3::Y,
            total_thickness_range.get_sub_range_with_steps(
                0,
                1,
                stem_count + 1,
            ),
            length,
            PineEntryType::Log,
            PineEntryType::Stem {
                branch_length: 10.,
                branch_thickness: total_branch_thickness_range
                    .get_sub_range_with_steps(0, stem_count, stem_count),
            },
        ));

        for i in 0..stem_count {
            let length = (rng.random_range(2.5..3.5) / VOXEL_SIZE) as usize;

            entries.extend(Self::create_straight_piece_dir(
                position + Vec3::Y * last_length as f32,
                Vec3::Y,
                total_thickness_range.get_sub_range_with_steps(
                    i,
                    i + 1,
                    stem_count + 1,
                ),
                length,
                PineEntryType::Log,
                PineEntryType::Stem {
                    branch_length: stem_count as f32 - i as f32 + 2.,
                    branch_thickness: total_branch_thickness_range
                        .get_sub_range_with_steps(i, stem_count, stem_count),
                },
            ));

            last_length += length;
        }

        entries
    }

    fn process_tree(
        mut start_state: &mut Vec<LSystemEntry<PineEntryType>>,
        rng: &mut StdRng,
        options: &PineOptions,
    ) {
        while Self::recurse_l_system(&mut start_state, rng, options) {}
    }

    fn get_block_from_entry(entry: &LSystemEntry<PineEntryType>) -> BlockType {
        match entry.entry_type {
            PineEntryType::Needle => BlockType::PineNeedle,
            _ => BlockType::PineLog,
        }
    }

    fn recurse_entry(
        entry: &LSystemEntry<PineEntryType>,
        rng: &mut StdRng,
        branches: &mut Vec<LSystemEntry<PineEntryType>>,
        _: &PineOptions,
    ) {
        match entry.entry_type {
            PineEntryType::Stem {
                branch_length,
                branch_thickness,
            } => {
                let branch_count = rng.random_range(4..=6);
                let angle_range: EntryRange = (0.0..360.).into();
                let angle_offset = angle_range.rng(rng);
                let random_angle_offset_range: EntryRange =
                    (-10.0..10.0).into();
                let branch_piece_length = 2.5;

                for i in 0..branch_count {
                    let angle_uncap = angle_range
                        .get_value_with_steps(i, branch_count)
                        + angle_offset
                        + random_angle_offset_range.rng(rng);
                    let angle = angle_uncap % 360.;

                    let down_angle = rng.random_range(-10.0..10.0);
                    let mut direction = Vec3::X;
                    let length_range =
                        (branch_length - 1.)..(branch_length + 1.);
                    let length = rng.random_range(length_range);

                    direction = rotate_around(
                        &direction,
                        &Vec3::ZERO,
                        -down_angle,
                        &RotationDirection::Z,
                    );

                    direction = rotate_around(
                        &direction,
                        &Vec3::ZERO,
                        angle,
                        &RotationDirection::Y,
                    );

                    let mut branch_start_pos = entry.pos;

                    for j in 0..(length / branch_piece_length).ceil() as i32 {
                        let this_pice_length = (length
                            - (branch_piece_length * j as f32))
                            .min(branch_piece_length);

                        let start_percent =
                            (j as f32 * branch_piece_length) / length;
                        let end_percent =
                            (((j + 1) as f32 * branch_piece_length) / length)
                                .min(1.0);

                        let mut branch_direction = rotate_around(
                            &direction,
                            &Vec3::ZERO,
                            j as f32 * -20.,
                            &RotationDirection::Y,
                        );
                        branch_direction -= Vec3::Y * j as f32 * -0.1;
                        branch_direction = branch_direction.normalize();

                        branches.extend(Self::create_straight_piece_dir(
                            branch_start_pos,
                            branch_direction,
                            branch_thickness
                                .get_sub_range(start_percent, end_percent),
                            (this_pice_length / VOXEL_SIZE) as usize,
                            PineEntryType::SubBranch {
                                direction: branch_direction,
                                tip: false,
                            },
                            PineEntryType::Branch {
                                direction: branch_direction,
                                tip: true,
                                sub_length: (5. - j as f32) * 2.5,
                            },
                        ));

                        branch_start_pos += (branch_direction.normalize()
                            * branch_piece_length)
                            / VOXEL_SIZE;
                    }
                }
            }
            PineEntryType::Branch { .. } => {}
            PineEntryType::SubBranch { direction, tip } => {
                let random_angle = rng.random_range(90.0..=91.0);
                let needle_count = 3;

                let ortho = direction.any_orthonormal_vector();

                for i in 0..needle_count {
                    let rotation = Quat::from_axis_angle(
                        direction,
                        i as f32 * (PI * 2. / needle_count as f32)
                            + random_angle,
                    );
                    let mut needle_direction =
                        rotation.mul_vec3(ortho).normalize();
                    needle_direction =
                        (needle_direction + direction).normalize();

                    branches.extend(Self::create_straight_piece_dir(
                        entry.pos + needle_direction,
                        needle_direction,
                        (0.4..0.35).into(),
                        3,
                        PineEntryType::Needle,
                        PineEntryType::Needle,
                    ));
                }

                if tip {
                    branches.extend(Self::create_straight_piece_dir(
                        entry.pos + direction,
                        direction,
                        (0.4..0.35).into(),
                        4,
                        PineEntryType::Needle,
                        PineEntryType::Needle,
                    ));
                }

                branches.push(LSystemEntry {
                    pos: entry.pos,
                    thickness: entry.thickness,
                    entry_type: PineEntryType::Log,
                });
            }
            _ => {}
        }
    }
}
