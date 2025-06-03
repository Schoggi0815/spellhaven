use crate::{
    utils::{rotate_around, vec_round_to_int, RotationDirection},
    world_generation::chunk_generation::{BlockType, VOXEL_SIZE},
};
use bevy::math::Vec3;
use rand::{rngs::StdRng, Rng};

pub struct LSystemEntry<EntryEnum> {
    pub pos: Vec3,
    pub thickness: f32,
    pub entry_type: EntryEnum,
}

pub trait LSystem<EntryEnum: Clone + Copy> {
    fn grow_new<const XSIZE: usize, const YSIZE: usize, const ZSIZE: usize>(
        rng: &mut StdRng,
    ) -> Vec<Vec<Vec<BlockType>>> {
        let pos = Vec3::new(XSIZE as f32 / 2., 0., ZSIZE as f32 / 2.);
        let pos_offset = Vec3 {
            x: rng.random(),
            y: 0.,
            z: rng.random(),
        };
        let mut start_state = Self::get_start_state(pos + pos_offset, rng);

        Self::process_tree(&mut start_state, rng);

        let mut voxel_grid = vec![];

        for x in 0..ZSIZE {
            voxel_grid.push(vec![]);
            for y in 0..YSIZE {
                voxel_grid[x].push(vec![]);
                for _ in 0..XSIZE {
                    voxel_grid[x][y].push(BlockType::Air);
                }
            }
        }

        start_state.iter().for_each(|entry| {
            let entry_pos = entry.pos;
            let center = vec_round_to_int(&entry_pos);
            let thickness = (entry.thickness / VOXEL_SIZE).ceil() as i32;

            for x in -thickness..thickness {
                for y in -thickness..thickness {
                    for z in -thickness..thickness {
                        let current_pos_i =
                            center + (Vec3::new(x as f32, y as f32, z as f32)).as_ivec3();
                        let current_pos = current_pos_i.as_vec3();

                        if current_pos_i.x < 0
                            || current_pos_i.x >= XSIZE as i32
                            || current_pos_i.y < 0
                            || current_pos_i.y >= YSIZE as i32
                            || current_pos_i.z < 0
                            || current_pos_i.z >= ZSIZE as i32
                        {
                            continue;
                        }

                        if current_pos.distance_squared(entry_pos)
                            < entry.thickness * entry.thickness / VOXEL_SIZE
                        {
                            voxel_grid[current_pos_i.x as usize][current_pos_i.y as usize]
                                [current_pos_i.z as usize] = Self::get_block_from_entry(entry);
                        }
                    }
                }
            }
        });

        voxel_grid
    }

    fn recurse_l_system(data: &mut Vec<LSystemEntry<EntryEnum>>, rng: &mut StdRng) {
        let mut i = 0usize;
        while i < data.len() {
            let entry = &data[i];
            let mut branches: Vec<LSystemEntry<EntryEnum>> = vec![];

            Self::recurse_entry(entry, rng, &mut branches);

            let length = branches.len();

            if length > 0 {
                data.splice(i..i + 1, branches);
                i += length;    
            }

            i += 1;
        }
    }

    fn create_straight_piece(
        pos: &Vec3,
        angle_x: f32,
        angle_z: f32,
        thickness: f32,
        length: usize,
        between_piece: EntryEnum,
        tip_piece: EntryEnum,
    ) -> Vec<LSystemEntry<EntryEnum>> {
        let mut pieces = Vec::new();

        for i in 0..length {
            pieces.push(LSystemEntry {
                pos: rotate_around(
                    &rotate_around(
                        &(*pos + (Vec3::Y * (i as f32))),
                        pos,
                        angle_z,
                        &RotationDirection::Z,
                    ),
                    pos,
                    angle_x,
                    &RotationDirection::X,
                ),
                entry_type: between_piece,
                thickness,
            });
        }
        pieces.push(LSystemEntry {
            pos: rotate_around(
                &rotate_around(
                    &(*pos + (Vec3::Y * (length as f32))),
                    pos,
                    angle_z,
                    &RotationDirection::Z,
                ),
                pos,
                angle_x,
                &RotationDirection::X,
            ),
            entry_type: tip_piece,
            thickness,
        });
        
        pieces
    }

    fn get_start_state(position: Vec3, rng: &mut StdRng) -> Vec<LSystemEntry<EntryEnum>>;
    fn process_tree(start_state: &mut Vec<LSystemEntry<EntryEnum>>, rng: &mut StdRng);
    fn get_block_from_entry(entry: &LSystemEntry<EntryEnum>) -> BlockType;
    fn recurse_entry(entry: &LSystemEntry<EntryEnum>, rng: &mut StdRng, branches: &mut Vec<LSystemEntry<EntryEnum>>);
}