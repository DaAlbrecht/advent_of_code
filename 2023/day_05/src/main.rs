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
    id: u32,
    soil: Option<u32>,
    fertilizer: Option<u32>,
    water: Option<u32>,
    light: Option<u32>,
    temperature: Option<u32>,
    humidity: Option<u32>,
    location: Option<u32>,
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
    sources: Vec<u32>,
    targets: Vec<u32>,
}

impl From<(u32, u32, u32)> for GardenMap {
    fn from((source, target, range): (u32, u32, u32)) -> Self {
        let targets = (target..target + range).collect::<Vec<_>>();
        let sources = (source..source + range).collect::<Vec<_>>();
        Self { sources, targets }
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle").unwrap();
    print!("part 01: {}", part_01(&input));
}

fn part_01(input: &str) -> u32 {
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
            id: *seed,
            ..Default::default()
        })
        .collect::<Vec<_>>()
}

fn generate_seed_for_map(seed: &mut Seed, map: &Maps) {
    match map {
        Maps::Seed2Soil(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                if garden_map.sources.contains(&seed.id) {
                    let index = garden_map
                        .sources
                        .iter()
                        .position(|&x| x == seed.id)
                        //seed needs to be in the sources
                        .unwrap();
                    seed.soil = Some(garden_map.targets[index]);
                } else {
                    seed.soil = Some(seed.id);
                }
            });
        }
        Maps::Soil2Fertilizer(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                if garden_map.sources.contains(&seed.soil.unwrap()) {
                    let index = garden_map
                        .sources
                        .iter()
                        .position(|&x| x == seed.soil.unwrap())
                        //seed needs to be in the sources
                        .unwrap();
                    seed.fertilizer = Some(garden_map.targets[index]);
                } else {
                    seed.fertilizer = Some(seed.soil.unwrap());
                }
            });
        }
        Maps::Fertilizer2Water(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                if garden_map.sources.contains(&seed.fertilizer.unwrap()) {
                    let index = garden_map
                        .sources
                        .iter()
                        .position(|&x| x == seed.fertilizer.unwrap())
                        //seed needs to be in the sources
                        .unwrap();
                    seed.water = Some(garden_map.targets[index]);
                } else {
                    seed.water = Some(seed.fertilizer.unwrap());
                }
            });
        }
        Maps::Water2Light(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                if garden_map.sources.contains(&seed.water.unwrap()) {
                    let index = garden_map
                        .sources
                        .iter()
                        .position(|&x| x == seed.water.unwrap())
                        //seed needs to be in the sources
                        .unwrap();
                    seed.light = Some(garden_map.targets[index]);
                } else {
                    seed.light = Some(seed.water.unwrap());
                }
            });
        }
        Maps::Light2Temperature(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                if garden_map.sources.contains(&seed.light.unwrap()) {
                    let index = garden_map
                        .sources
                        .iter()
                        .position(|&x| x == seed.light.unwrap())
                        //seed needs to be in the sources
                        .unwrap();
                    seed.temperature = Some(garden_map.targets[index]);
                } else {
                    seed.temperature = Some(seed.light.unwrap());
                }
            });
        }
        Maps::Temperature2Humidity(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                if garden_map.sources.contains(&seed.temperature.unwrap()) {
                    let index = garden_map
                        .sources
                        .iter()
                        .position(|&x| x == seed.temperature.unwrap())
                        //seed needs to be in the sources
                        .unwrap();
                    seed.humidity = Some(garden_map.targets[index]);
                } else {
                    seed.humidity = Some(seed.temperature.unwrap());
                }
            });
        }
        Maps::Humidity2Location(garden_maps) => {
            garden_maps.iter().for_each(|garden_map| {
                if garden_map.sources.contains(&seed.humidity.unwrap()) {
                    let index = garden_map
                        .sources
                        .iter()
                        .position(|&x| x == seed.humidity.unwrap())
                        //seed needs to be in the sources
                        .unwrap();
                    seed.location = Some(garden_map.targets[index]);
                } else {
                    seed.location = Some(seed.humidity.unwrap());
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
}
