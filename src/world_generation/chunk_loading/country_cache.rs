use crate::world_generation::chunk_generation::noise::full_cache::FullCache;
use crate::world_generation::chunk_generation::noise::lod_height_adjuster::LodHeightAdjuster;
use crate::world_generation::chunk_generation::voxel_generation::get_terrain_noise;
use crate::world_generation::chunk_generation::BlockType;
use crate::world_generation::generation_options::{GenerationCacheItem, GenerationOptions};
use crate::world_generation::voxel_world::ChunkLod;
use bevy::log::info;
use bevy::math::{IVec2, Vec2};
use noise::NoiseFn;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct CountryCache {
    pub country_pos: IVec2,
    pub grass_color: BlockType,
    pub structure_cache: Arc<StructureCache>,
    pub this_path_cache: Arc<PathCache>,
    pub bottom_path_cache: Arc<PathCache>,
    pub left_path_cache: Arc<PathCache>,
}

pub struct StructureCache {
    pub city_location: IVec2,
}

pub struct PathCache {
    pub paths: Vec<Path>,
}

pub struct Path {
    pub lines: Vec<PathLine>,
    pub box_pos_start: IVec2,
    pub box_pos_end: IVec2,
}

impl Path {
    pub fn is_in_box(&self, point: IVec2, margin: IVec2) -> bool {
        let bb_start = self.box_pos_start - margin;
        let bb_end = self.box_pos_end + margin;
        !(point.x < bb_start.x || point.x > bb_end.x || point.y < bb_start.y || point.y > bb_end.y)
    }
}

pub struct PathLine {
    pub start: IVec2,
    pub end: IVec2,
    pub spline_one: Vec2,
    pub spline_two: Vec2,
    pub box_pos_start: IVec2,
    pub box_pos_end: IVec2,
    pub estimated_length: f32,
    pub sample_points: Vec<IVec2>,
}

impl PathLine {
    fn new(start: IVec2, end: IVec2, before: IVec2, after: IVec2) -> Self {
        let spline_one = start.as_vec2() + (end - before).as_vec2() / 2. / 3.;
        let spline_two = end.as_vec2() - (after - start).as_vec2() / 2. / 3.;

        let estimated_length = start.as_vec2().distance(end.as_vec2());

        let spline_one =
            start.as_vec2() + (spline_one - start.as_vec2()).normalize() * (estimated_length / 2.);
        let spline_two =
            end.as_vec2() + (spline_two - end.as_vec2()).normalize() * (estimated_length / 2.);

        let box_pos_start = start.min(end);
        let box_pos_end = start.max(end);

        let mut path_line = Self {
            start,
            end,
            spline_one,
            spline_two,
            box_pos_start,
            box_pos_end,
            estimated_length,
            sample_points: vec![start],
        };

        let num_points = (estimated_length / 20.).max(2.);

        let mut last_point = IVec2::ZERO;

        for i in 1..num_points as i32 {
            let current_progress = i as f32 / num_points;

            let current_pos = path_line.lerp_on_spline(current_progress).as_ivec2();

            if last_point != current_pos {
                path_line.sample_points.push(current_pos);
                path_line.box_pos_start = path_line.box_pos_start.min(current_pos);
                path_line.box_pos_end = path_line.box_pos_end.max(current_pos);

                last_point = current_pos;
            }
        }

        path_line.sample_points.push(end);

        path_line
    }

    pub fn is_in_box(&self, point: IVec2, margin: IVec2) -> bool {
        let bb_start = self.box_pos_start - margin;
        let bb_end = self.box_pos_end + margin;
        !(point.x < bb_start.x || point.x > bb_end.x || point.y < bb_start.y || point.y > bb_end.y)
    }

    pub fn get_progress_on_line(&self, point: IVec2) -> f32 {
        let distance_to_start = self.start.distance_squared(point) as f32;
        let distance_to_end = self.end.distance_squared(point) as f32;

        distance_to_start / (distance_to_start + distance_to_end)
    }

    pub fn closest_point_on_path(&self, point: IVec2, margin: IVec2) -> Option<(Vec2, Vec2)> {
        let mut min_squared = i32::MAX;
        let mut closest = Vec2::ZERO;
        let mut closest_index = 0usize;

        for i in 1..self.sample_points.len() {
            let end = self.sample_points[i];
            let start = self.sample_points[i - 1];

            let box_start = start.min(end);
            let box_end = start.max(end);

            if point.cmpge(box_start - margin).all() && point.cmplt(box_end + margin).all() {
                let closest_point = Self::get_closest_point_to_line(start, end, point);
                let dist_squared = point.distance_squared(closest_point.as_ivec2());
                if dist_squared < min_squared {
                    min_squared = dist_squared;
                    closest = closest_point;
                    closest_index = i;
                }
            }
        }

        if min_squared < i32::MAX {
            let closest_start = self.sample_points[closest_index - 1];
            let closest_end = self.sample_points[closest_index];

            Some((closest, (closest_end - closest_start).as_vec2().normalize()))
        } else {
            None
        }
    }

    fn get_closest_point_to_line(line_start: IVec2, line_end: IVec2, point: IVec2) -> Vec2 {
        let length_squared = (line_end - line_start).length_squared();
        if length_squared == 0 {
            return line_start.as_vec2();
        }

        let t = ((point - line_start).dot(line_end - line_start) as f32 / length_squared as f32)
            .clamp(0., 1.);
        let projection = line_start.as_vec2() + t * (line_end - line_start).as_vec2();

        projection
    }

    pub fn lerp_on_spline(&self, t: f32) -> Vec2 {
        let a = Self::lerp(self.start.as_vec2(), self.spline_one, t);
        let b = Self::lerp(self.spline_one, self.spline_two, t);
        let c = Self::lerp(self.spline_two, self.end.as_vec2(), t);

        let d = Self::lerp(a, b, t);
        let e = Self::lerp(b, c, t);

        Self::lerp(d, e, t)
    }

    fn lerp(p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
        (1. - t) * p1 + t * p2
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AStarCandidate {
    estimated_weight: i32,
    real_weight: i32,
    state: IVec2,
    direction: IVec2,
}

impl PartialOrd for AStarCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarCandidate {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimated_weight.cmp(&self.estimated_weight)
    }
}

impl GenerationCacheItem<IVec2> for CountryCache {
    fn generate(key: IVec2, generation_options: &GenerationOptions) -> Self {
        let mut rng = rand::rng();

        Self {
            country_pos: key,
            grass_color: BlockType::Custom(rng.random(), rng.random(), rng.random()),
            structure_cache: generation_options
                .structure_cache
                .get_cache_entry(key, generation_options),
            this_path_cache: generation_options
                .path_cache
                .get_cache_entry(key, generation_options),
            bottom_path_cache: generation_options
                .path_cache
                .get_cache_entry(key + IVec2::NEG_X, generation_options),
            left_path_cache: generation_options
                .path_cache
                .get_cache_entry(key + IVec2::NEG_Y, generation_options),
        }
    }
}

impl GenerationCacheItem<IVec2> for StructureCache {
    fn generate(key: IVec2, generation_options: &GenerationOptions) -> Self {
        let mut rng = StdRng::seed_from_u64(if key.x < 0 {
            generation_options.seed.wrapping_sub(key.x.abs() as u64)
        } else {
            generation_options.seed.wrapping_add(key.x.abs() as u64)
        });
        let mut rng = StdRng::seed_from_u64(if key.x < 0 {
            rng.random::<u64>().wrapping_sub(key.y.abs() as u64)
        } else {
            rng.random::<u64>().wrapping_add(key.y.abs() as u64)
        });

        let min_offset = 100i32;

        let city_x = rng.random_range(min_offset..COUNTRY_SIZE as i32 - min_offset);
        let city_z = rng.random_range(min_offset..COUNTRY_SIZE as i32 - min_offset);

        Self {
            city_location: IVec2::new(city_x, city_z) + key * COUNTRY_SIZE as i32,
        }
    }
}

impl GenerationCacheItem<IVec2> for PathCache {
    fn generate(key: IVec2, generation_options: &GenerationOptions) -> Self {
        let top_country_pos = key + IVec2::X;
        let right_country_pos = key + IVec2::Y;

        let current_structure_cache = generation_options
            .structure_cache
            .get_cache_entry(key, generation_options);
        let top_structure_cache = generation_options
            .structure_cache
            .get_cache_entry(top_country_pos, generation_options);
        let right_structure_cache = generation_options
            .structure_cache
            .get_cache_entry(right_country_pos, generation_options);

        let path_finding_lod = ChunkLod::Sixteenth;

        Self {
            paths: vec![
                PathCache::generate_path(
                    current_structure_cache.city_location,
                    top_structure_cache.city_location,
                    [key, top_country_pos],
                    path_finding_lod,
                    generation_options,
                ),
                PathCache::generate_path(
                    current_structure_cache.city_location,
                    right_structure_cache.city_location,
                    [key, right_country_pos],
                    path_finding_lod,
                    generation_options,
                ),
            ],
        }
    }
}

impl PathCache {
    fn generate_path(
        mut start_pos: IVec2,
        mut end_pos: IVec2,
        country_positions: [IVec2; 2],
        path_finding_lod: ChunkLod,
        generation_options: &GenerationOptions,
    ) -> Path {
        // return Path {
        //     lines: vec![],
        //     box_pos_start: Default::default(),
        //     box_pos_end: Default::default(),
        // };

        start_pos /= path_finding_lod.multiplier_i32();
        end_pos /= path_finding_lod.multiplier_i32();

        let terrain_noise = FullCache::new(LodHeightAdjuster::new(
            get_terrain_noise(generation_options),
            path_finding_lod,
        ));

        let get_terrain_height = |pos: IVec2| -> f64 {
            terrain_noise.get(
                (pos * path_finding_lod.multiplier_i32())
                    .as_dvec2()
                    .to_array(),
            ) * path_finding_lod.multiplier_i32() as f64
        };

        let distance_to_end = |pos: IVec2| -> i32 {
            let diff = (end_pos - pos).abs();
            let smaller = if diff.x < diff.y { diff.x } else { diff.y };
            let bigger = if diff.x > diff.y { diff.x } else { diff.y };
            bigger * 10 + smaller * 4
        };

        let neighbours = |pos: IVec2| -> [(IVec2, i32); 8] {
            [
                (pos + IVec2::new(1, 0), 10),
                (pos + IVec2::new(0, 1), 10),
                (pos + IVec2::new(-1, 0), 10),
                (pos + IVec2::new(0, -1), 10),
                (pos + IVec2::new(1, 1), 14),
                (pos + IVec2::new(-1, 1), 14),
                (pos + IVec2::new(-1, -1), 14),
                (pos + IVec2::new(1, -1), 14),
            ]
        };

        let is_outside_of_countries = |pos: IVec2| -> bool {
            let pos = pos * path_finding_lod.multiplier_i32();
            let is_outside_first_country = pos.x < country_positions[0].x * COUNTRY_SIZE as i32
                || pos.x >= (country_positions[0] + IVec2::X).x * COUNTRY_SIZE as i32
                || pos.y < country_positions[0].y * COUNTRY_SIZE as i32
                || pos.y >= (country_positions[0] + IVec2::Y).y * COUNTRY_SIZE as i32;
            let is_outside_second_country = pos.x < country_positions[1].x * COUNTRY_SIZE as i32
                || pos.x >= (country_positions[1] + IVec2::X).x * COUNTRY_SIZE as i32
                || pos.y < country_positions[1].y * COUNTRY_SIZE as i32
                || pos.y >= (country_positions[1] + IVec2::Y).y * COUNTRY_SIZE as i32;
            is_outside_first_country && is_outside_second_country
        };

        let mut queue = BinaryHeap::new();
        let mut previous = HashMap::new();
        let mut weights = HashMap::new();

        weights.insert(start_pos, 0);
        queue.push(AStarCandidate {
            estimated_weight: distance_to_end(start_pos),
            real_weight: 0,
            state: start_pos,
            direction: IVec2::ZERO,
        });

        info!("start_pos: {start_pos}, end_pos: {end_pos}");

        let now = Instant::now();

        while let Some(AStarCandidate {
            estimated_weight: _,
            real_weight,
            state: current,
            direction: current_direction,
        }) = queue.pop()
        {
            if current == end_pos {
                break;
            }

            let current_height = get_terrain_height(current);

            for (next, weight) in neighbours(current) {
                if is_outside_of_countries(next) {
                    continue;
                }

                let direction = next - current;
                let direction_difference = (direction - current_direction).abs();
                let direction_cost = direction_difference.x + direction_difference.y;

                if direction_cost > 1 {
                    continue;
                }

                let next_height = get_terrain_height(next);

                let height_difference =
                    (current_height - next_height).abs() / path_finding_lod.multiplier_i32() as f64;
                if height_difference > 0.65 {
                    continue;
                }

                let direction_turned = direction.perp();
                let side_height = get_terrain_height(next + direction_turned);
                let steepness =
                    (next_height - side_height).abs() / path_finding_lod.multiplier_i32() as f64;

                let real_weight = real_weight
                    + weight
                    + (height_difference * 30.) as i32
                    + (steepness * 20.) as i32; //((total_steepness * 0.6).max(0.) * 10.0) as i32;
                if weights
                    .get(&next)
                    .map(|&weight| real_weight < weight)
                    .unwrap_or(true)
                {
                    let estimated_weight = real_weight + distance_to_end(next);
                    weights.insert(next, real_weight);
                    queue.push(AStarCandidate {
                        estimated_weight,
                        real_weight,
                        state: next,
                        direction,
                    });
                    previous.insert(next, current);
                }
            }
        }

        let elapsed = now.elapsed();

        info!("DONE: {}s", elapsed.as_secs_f32());

        if previous.get(&end_pos).is_some() {
            let mut min_x = 0;
            let mut min_y = 0;
            let mut max_x = 0;
            let mut max_y = 0;

            let mut check_min_max = |pos: IVec2| {
                min_x = min_x.min(pos.x);
                min_y = min_y.min(pos.y);
                max_x = max_x.max(pos.x);
                max_y = max_y.max(pos.y);
            };

            let mut current = end_pos;
            let mut path: Vec<PathLine> = vec![];

            let mut points: Vec<IVec2> = vec![];

            if let Some(parent) = previous.get(&current) {
                points.push((current - (*parent - current)) * path_finding_lod.multiplier_i32());
            }

            while current != start_pos {
                let prev = previous
                    .get(&current)
                    .copied()
                    .expect("We reached the target, but are unable to reconsistute the path");

                let dir = prev - current;

                let next = current * path_finding_lod.multiplier_i32()
                    + (dir * path_finding_lod.multiplier_i32()) / 2;

                points.push(next);

                check_min_max(next);

                current = prev;
            }

            // let mut direction = IVec2::new(0, 0);

            // let mut chopped_points = Vec::new();
            //
            // for i in 1..points.len() {
            //     let dir = points[i] - points[i - 1];
            //     if dir != direction {
            //         chopped_points.push(points[i - 1]);
            //
            //         direction = dir;
            //     }
            // }
            //
            // chopped_points.push(*points.last().unwrap_or(&IVec2::default()));

            //let points = chopped_points;

            let last = current * path_finding_lod.multiplier_i32();
            //points.push(last);
            //check_min_max(last);

            if points.len() >= 4 {
                points.push(last - (points[points.len() - 2] - last));

                for i in 1..points.len() - 2 {
                    path.push(PathLine::new(
                        points[i],
                        points[i + 1],
                        points[i - 1],
                        points[i + 2],
                    ));
                }
            }

            Path {
                lines: path,
                box_pos_start: IVec2::new(min_x, min_y),
                box_pos_end: IVec2::new(max_x, max_y),
            }
        } else {
            info!("NO PATH COULD BE CREATED!");
            Path {
                lines: vec![],
                box_pos_start: Default::default(),
                box_pos_end: Default::default(),
            }
        }
    }
}

pub const COUNTRY_SIZE: usize = 2usize.pow(15);
