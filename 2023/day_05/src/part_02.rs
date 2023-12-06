use crate::{parse_maps, Maps};
use rayon::prelude::*;

#[derive(Debug, Default)]
pub struct Seed {
    id: u64,
    soil: Vec<u64>,
    fertilizer: Vec<u64>,
    water: Vec<u64>,
    light: Vec<u64>,
    temperature: Vec<u64>,
    humidity: Vec<u64>,
    location: Vec<u64>,
}

pub fn part_02(input: &str) -> u64 {
    let maps = parse_maps(input);
    let seed_ranges = parse_seed_ranges(input);

    seed_ranges
        .par_iter()
        .map(|seed_range| {
            let mut min_locations = Vec::new();
            for seed in seed_range.0..seed_range.0 + seed_range.1 {
                let mut seed = Seed {
                    id: seed,
                    ..Default::default()
                };
                plann_seed(&mut seed, &maps);
                let location = *seed.location.iter().min().unwrap();
                min_locations.push(location);
            }
            min_locations
        })
        .flatten()
        .min()
        .unwrap()
}

fn is_in_range(value: u64, range: (u64, u64)) -> bool {
    range.0 <= value && value <= range.0 + range.1
}

fn plann_seed(seed: &mut Seed, maps: &Vec<Maps>) {
    for map in maps {
        match map {
            Maps::Seed2Soil(garden_map) => {
                for map in garden_map {
                    if is_in_range(seed.id, (map.source, map.range)) {
                        let difference = seed.id - map.source;
                        seed.soil.push(map.target + difference);
                    }
                }
                if seed.soil.is_empty() {
                    seed.soil.push(seed.id);
                }
            }
            Maps::Soil2Fertilizer(garden_map) => {
                for soil in &seed.soil {
                    for map in garden_map {
                        if is_in_range(*soil, (map.source, map.range)) {
                            let difference = soil - map.source;
                            seed.fertilizer.push(map.target + difference);
                        }
                    }
                    if seed.fertilizer.is_empty() {
                        seed.fertilizer.push(*soil);
                    }
                }
            }
            Maps::Fertilizer2Water(garden_map) => {
                for fertilizer in &seed.fertilizer {
                    for map in garden_map {
                        if is_in_range(*fertilizer, (map.source, map.range)) {
                            let difference = fertilizer - map.source;
                            seed.water.push(map.target + difference);
                        }
                    }
                    if seed.water.is_empty() {
                        seed.water.push(*fertilizer);
                    }
                }
            }
            Maps::Water2Light(garden_map) => {
                for water in &seed.water {
                    for map in garden_map {
                        if is_in_range(*water, (map.source, map.range)) {
                            let difference = water - map.source;
                            seed.light.push(map.target + difference);
                        }
                    }
                    if seed.light.is_empty() {
                        seed.light.push(*water);
                    }
                }
            }
            Maps::Light2Temperature(garden_map) => {
                for light in &seed.light {
                    for map in garden_map {
                        if is_in_range(*light, (map.source, map.range)) {
                            let difference = light - map.source;
                            seed.temperature.push(map.target + difference);
                        }
                    }
                    if seed.temperature.is_empty() {
                        seed.temperature.push(*light);
                    }
                }
            }
            Maps::Temperature2Humidity(garden_map) => {
                for temperature in &seed.temperature {
                    for map in garden_map {
                        if is_in_range(*temperature, (map.source, map.range)) {
                            let difference = temperature - map.source;
                            seed.humidity.push(map.target + difference);
                        }
                    }
                    if seed.humidity.is_empty() {
                        seed.humidity.push(*temperature);
                    }
                }
            }
            Maps::Humidity2Location(garden_map) => {
                for humidity in &seed.humidity {
                    for map in garden_map {
                        if is_in_range(*humidity, (map.source, map.range)) {
                            let difference = humidity - map.source;
                            seed.location.push(map.target + difference);
                        }
                    }
                    if seed.location.is_empty() {
                        seed.location.push(*humidity);
                    }
                }
            }
        }
    }
}

fn parse_seed_ranges(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .next()
        .expect("No input")
        .strip_prefix("seeds: ")
        .expect("expects line to start with 'seeds: '")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|seed| {
            let start = seed[0].parse::<u64>().expect("Unable to parse seed start");
            let range = seed[1].parse::<u64>().expect("Unable to parse seed range");
            (start, range)
        })
        .collect::<Vec<(u64, u64)>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_02() {
        let input = std::fs::read_to_string("test").expect("Unable to read file");
        assert_eq!(super::part_02(&input), 46);
    }
}
