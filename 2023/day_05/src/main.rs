use itertools::Itertools;

#[derive(Debug)]
enum Maps {
    Seed2Soil(Vec<GardenMap>),
    Soil2Fertilizer(Vec<GardenMap>),
    Fertilizer2Water(Vec<GardenMap>),
    Water2Light(Vec<GardenMap>),
    Light2Temperature(Vec<GardenMap>),
    Temperature2Humidity(Vec<GardenMap>),
    Humidity2Location(Vec<GardenMap>),
}

#[derive(Debug)]
struct Seed {
    id: u64,
    soil: Option<u64>,
    fertilizer: Option<u64>,
    water: Option<u64>,
    light: Option<u64>,
    temperature: Option<u64>,
    humidity: Option<u64>,
    location: Option<u64>,
}

impl Default for Seed {
    fn default() -> Self {
        Self {
            id: 0,
            soil: None,
            fertilizer: None,
            water: None,
            light: None,
            temperature: None,
            humidity: None,
            location: None,
        }
    }
}

#[derive(Debug)]
struct GardenMap {
    source: u64,
    target: u64,
    range: u64,
}

impl From<(u32, u32, u32)> for GardenMap {
    fn from((source, target, range): (u32, u32, u32)) -> Self {
        Self {
            source: source as u64,
            target: target as u64,
            range: range as u64,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle").unwrap();
    //print!("part 01: {}", part_01(&input));
    print!("part 02: {}", part_02(&input));
}

fn part_01(input: &str) -> u64 {
    let mut seeds = parse_seeds(input);
    let maps = parse_maps(input);
    maps.iter().for_each(|map| {
        seeds.iter_mut().for_each(|seed| {
            generate_seed_for_map(seed, map);
        });
    });

    seeds
        .iter()
        .map(|seed| seed.location.unwrap())
        .min()
        .unwrap()
}

fn part_02(input: &str) -> u64 {
    let mut seeds = parse_seeds_part02(input);
    let maps = parse_maps(input);
    maps.iter().for_each(|map| {
        seeds.iter_mut().for_each(|seed| {
            generate_seed_for_map(seed, map);
        });
    });

    seeds
        .iter()
        .map(|seed| seed.location.unwrap())
        .max()
        .unwrap()
}

fn parse_seeds_part02(input: &str) -> Vec<Seed> {
    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let chunks = line
                .strip_prefix("seeds: ")
                .expect("puzzle to start with 'Seeds: '")
                .split(' ')
                .chunks(2);

            let chunks = chunks.into_iter();

            let ids = chunks
                .map(|chunk| {
                    let mut chunk = chunk;
                    let seed = chunk
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .expect("seed to be a number");
                    let range = chunk
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .expect("range to be a number");
                    (seed..seed + range).collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>();
            ids.iter()
                .map(|id| Seed {
                    id: *id as u64,
                    ..Default::default()
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn parse_maps(input: &str) -> Vec<Maps> {
    input
        .split("\n\n")
        .skip(1)
        .map(|map| {
            let description = map
                .lines()
                .next()
                .expect("map to have a description")
                .strip_suffix(" map:")
                .expect("map to end with ' map:'");
            let map = map.lines().skip(1).collect::<Vec<_>>();
            let maps = map
                .iter()
                .filter_map(|line| {
                    let mut line = line.split(' ');
                    let target = line.next()?.parse::<u32>().expect("source to be a number");
                    let source = line.next()?.parse::<u32>().expect("target to be a number");
                    let range = line.next()?.parse::<u32>().expect("range to be a number");
                    Some((source, target, range))
                })
                .collect::<Vec<_>>();
            match description {
                "seed-to-soil" => {
                    let garden_maps = maps
                        .iter()
                        .map(|(source, target, range)| GardenMap::from((*source, *target, *range)))
                        .collect::<Vec<_>>();
                    Maps::Seed2Soil(garden_maps)
                }
                "soil-to-fertilizer" => {
                    let garden_maps = maps
                        .iter()
                        .map(|(source, target, range)| GardenMap::from((*source, *target, *range)))
                        .collect::<Vec<_>>();
                    Maps::Soil2Fertilizer(garden_maps)
                }
                "fertilizer-to-water" => {
                    let garden_maps = maps
                        .iter()
                        .map(|(source, target, range)| GardenMap::from((*source, *target, *range)))
                        .collect::<Vec<_>>();
                    Maps::Fertilizer2Water(garden_maps)
                }
                "water-to-light" => {
                    let garden_maps = maps
                        .iter()
                        .map(|(source, target, range)| GardenMap::from((*source, *target, *range)))
                        .collect::<Vec<_>>();
                    Maps::Water2Light(garden_maps)
                }
                "light-to-temperature" => {
                    let garden_maps = maps
                        .iter()
                        .map(|(source, target, range)| GardenMap::from((*source, *target, *range)))
                        .collect::<Vec<_>>();
                    Maps::Light2Temperature(garden_maps)
                }
                "temperature-to-humidity" => {
                    let garden_maps = maps
                        .iter()
                        .map(|(source, target, range)| GardenMap::from((*source, *target, *range)))
                        .collect::<Vec<_>>();
                    Maps::Temperature2Humidity(garden_maps)
                }
                "humidity-to-location" => {
                    let garden_maps = maps
                        .iter()
                        .map(|(source, target, range)| GardenMap::from((*source, *target, *range)))
                        .collect::<Vec<_>>();
                    Maps::Humidity2Location(garden_maps)
                }
                _ => panic!("unknown map"),
            }
        })
        .collect::<Vec<_>>()
}

fn parse_seeds(input: &str) -> Vec<Seed> {
    let seeds = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.strip_prefix("seeds: ")
                .expect("puzzle to start with 'Seeds: '")
                .split(' ')
                .map(|seed| seed.parse::<u32>().expect("seed to be a number"))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    seeds
        .iter()
        .map(|seed| Seed {
            id: *seed as u64,
            ..Default::default()
        })
        .collect::<Vec<_>>()
}

fn generate_seed_for_map(seed: &mut Seed, map: &Maps) {
    match map {
        Maps::Seed2Soil(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                //check if source is in the range of garden_map.source .. garden_map.source + garden_map.range
                if garden_map.source <= seed.id && seed.id <= garden_map.source + garden_map.range {
                    let difference = seed.id - garden_map.source;
                    seed.soil = Some(garden_map.target as u64 + difference as u64);
                } else if seed.soil.is_none() {
                    seed.soil = Some(seed.id as u64);
                }
            });
        }
        Maps::Soil2Fertilizer(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                //check if source is in the range of garden_map.source .. garden_map.source + garden_map.range
                if garden_map.source <= seed.soil.unwrap()
                    && seed.soil.unwrap() <= garden_map.source + garden_map.range
                {
                    let difference = seed.soil.unwrap() - garden_map.source;
                    seed.fertilizer = Some(garden_map.target as u64 + difference as u64);
                } else if seed.fertilizer.is_none() {
                    seed.fertilizer = Some(seed.soil.unwrap() as u64);
                }
            });
        }
        Maps::Fertilizer2Water(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                //check if source is in the range of garden_map.source .. garden_map.source + garden_map.range
                if garden_map.source <= seed.fertilizer.unwrap()
                    && seed.fertilizer.unwrap() <= garden_map.source + garden_map.range
                {
                    let difference = seed.fertilizer.unwrap() - garden_map.source;
                    seed.water = Some(garden_map.target as u64 + difference as u64);
                } else if seed.water.is_none() {
                    seed.water = Some(seed.fertilizer.unwrap() as u64);
                }
            });
        }
        Maps::Water2Light(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                //check if source is in the range of garden_map.source .. garden_map.source + garden_map.range
                if garden_map.source <= seed.water.unwrap()
                    && seed.water.unwrap() <= garden_map.source + garden_map.range
                {
                    let difference = seed.water.unwrap() - garden_map.source;
                    seed.light = Some(garden_map.target as u64 + difference as u64);
                } else if seed.light.is_none() {
                    seed.light = Some(seed.water.unwrap() as u64);
                }
            });
        }
        Maps::Light2Temperature(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                //check if source is in the range of garden_map.source .. garden_map.source + garden_map.range
                if garden_map.source <= seed.light.unwrap()
                    && seed.light.unwrap() <= garden_map.source + garden_map.range
                {
                    let difference = seed.light.unwrap() - garden_map.source;
                    seed.temperature = Some(garden_map.target as u64 + difference as u64);
                } else if seed.temperature.is_none() {
                    seed.temperature = Some(seed.light.unwrap() as u64);
                }
            });
        }
        Maps::Temperature2Humidity(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                //check if source is in the range of garden_map.source .. garden_map.source + garden_map.range
                if garden_map.source <= seed.temperature.unwrap()
                    && seed.temperature.unwrap() <= garden_map.source + garden_map.range
                {
                    let difference = seed.temperature.unwrap() - garden_map.source;
                    seed.humidity = Some(garden_map.target as u64 + difference as u64);
                } else if seed.humidity.is_none() {
                    seed.humidity = Some(seed.temperature.unwrap() as u64);
                }
            });
        }
        Maps::Humidity2Location(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                //check if source is in the range of garden_map.source .. garden_map.source + garden_map.range
                if garden_map.source <= seed.humidity.unwrap()
                    && seed.humidity.unwrap() <= garden_map.source + garden_map.range
                {
                    let difference = seed.humidity.unwrap() - garden_map.source;
                    seed.location = Some(garden_map.target as u64 + difference as u64);
                } else if seed.location.is_none() {
                    seed.location = Some(seed.humidity.unwrap() as u64);
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn part_01() {
        let input = std::fs::read_to_string("test").unwrap();
        assert_eq!(crate::part_01(&input), 35);
    }
    #[test]
    fn part_02() {
        let input = std::fs::read_to_string("test").unwrap();
        assert_eq!(crate::part_02(&input), 46);
    }
}
