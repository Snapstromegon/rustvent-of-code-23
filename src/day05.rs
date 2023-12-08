use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    fn part1(&self, input: &str) -> Option<usize> {
        let mut blocks = input.split("\r\n\r\n");
        let seeds = blocks
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mappings: Vec<(&str, Map)> = blocks.map(|block| parse_mapping(block)).collect();
        let alm = Almanac {
            seed_to_soil: mappings[0].1.clone(),
            soil_to_fertilizer: mappings[1].1.clone(),
            fertilizer_to_water: mappings[2].1.clone(),
            water_to_light: mappings[3].1.clone(),
            light_to_temperature: mappings[4].1.clone(),
            temperature_to_humidity: mappings[5].1.clone(),
            humidity_to_location: mappings[6].1.clone(),
        };
        let mut sorted = seeds
            .iter()
            .map(|seed| alm.map_seed_to_location(*seed))
            .collect::<Vec<usize>>();
        sorted.sort();

        Some(sorted[0])
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let mut blocks = input.split("\r\n\r\n");
        let seed_pairs = blocks
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .chunks_exact(2)
            .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
            .collect::<Vec<(usize, usize)>>();
        let mappings: Vec<(&str, Map)> = blocks.map(|block| parse_mapping(block)).collect();
        let alm = Almanac {
            seed_to_soil: mappings[0].1.clone(),
            soil_to_fertilizer: mappings[1].1.clone(),
            fertilizer_to_water: mappings[2].1.clone(),
            water_to_light: mappings[3].1.clone(),
            light_to_temperature: mappings[4].1.clone(),
            temperature_to_humidity: mappings[5].1.clone(),
            humidity_to_location: mappings[6].1.clone(),
        };
        for i in 0..100 {
            assert!(alm.seed_to_soil.map(i) == alm.map_seed_to_soil(i));
            assert!(alm.seed_to_soil.unmap(i) == alm.map_soil_to_seed(i));
            assert!(alm.soil_to_fertilizer.map(alm.seed_to_soil.map(i)) == alm.map_seed_to_fertilizer(i));
            assert!(alm.seed_to_soil.unmap(alm.soil_to_fertilizer.unmap(i)) == alm.map_fertilizer_to_seed(i));
            assert!(alm.fertilizer_to_water.map(alm.soil_to_fertilizer.map(alm.seed_to_soil.map(i))) == alm.map_seed_to_water(i));
            assert!(alm.seed_to_soil.unmap(alm.soil_to_fertilizer.unmap(alm.fertilizer_to_water.unmap(i))) == alm.map_water_to_seed(i));
            assert!(alm.water_to_light.map(alm.fertilizer_to_water.map(alm.soil_to_fertilizer.map(alm.seed_to_soil.map(i)))) == alm.map_seed_to_light(i));
            assert!(alm.seed_to_soil.unmap(alm.soil_to_fertilizer.unmap(alm.fertilizer_to_water.unmap(alm.water_to_light.unmap(i)))) == alm.map_light_to_seed(i));
            assert!(alm.light_to_temperature.map(alm.water_to_light.map(alm.fertilizer_to_water.map(alm.soil_to_fertilizer.map(alm.seed_to_soil.map(i))))) == alm.map_seed_to_temperature(i));
            assert!(alm.seed_to_soil.unmap(alm.soil_to_fertilizer.unmap(alm.fertilizer_to_water.unmap(alm.water_to_light.unmap(alm.light_to_temperature.unmap(i))))) == alm.map_temperature_to_seed(i));
            assert!(alm.temperature_to_humidity.map(alm.light_to_temperature.map(alm.water_to_light.map(alm.fertilizer_to_water.map(alm.soil_to_fertilizer.map(alm.seed_to_soil.map(i)))))) == alm.map_seed_to_humidity(i));
            assert!(alm.seed_to_soil.unmap(alm.soil_to_fertilizer.unmap(alm.fertilizer_to_water.unmap(alm.water_to_light.unmap(alm.light_to_temperature.unmap(alm.temperature_to_humidity.unmap(i)))))) == alm.map_humidity_to_seed(i));
            assert!(alm.humidity_to_location.map(alm.temperature_to_humidity.map(alm.light_to_temperature.map(alm.water_to_light.map(alm.fertilizer_to_water.map(alm.soil_to_fertilizer.map(alm.seed_to_soil.map(i))))))) == alm.map_seed_to_location(i));
            assert!(alm.seed_to_soil.unmap(alm.soil_to_fertilizer.unmap(alm.fertilizer_to_water.unmap(alm.water_to_light.unmap(alm.light_to_temperature.unmap(alm.temperature_to_humidity.unmap(alm.humidity_to_location.unmap(i))))))) == alm.map_location_to_seed(i));
        }



        for seed_pair in seed_pairs {
            println!(
                "pair: {:?}, splits: {:?}",
                seed_pair,
                alm.suggest_splits(seed_pair)
            );
        }
        None
    }
}

fn parse_mapping(block: &str) -> (&str, Map) {
    let (name, rest) = block.split_once(':').unwrap();
    let mappings = rest
        .trim()
        .split("\r\n")
        .map(|line| {
            let mut entries = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            Mapping {
                dest: entries.next().unwrap(),
                source: entries.next().unwrap(),
                length: entries.next().unwrap(),
            }
        })
        .collect::<Vec<Mapping>>();
    (name, Map { mappings })
}

#[derive(Debug, Clone)]
struct Almanac {
    pub seed_to_soil: Map,
    pub soil_to_fertilizer: Map,
    pub fertilizer_to_water: Map,
    pub water_to_light: Map,
    pub light_to_temperature: Map,
    pub temperature_to_humidity: Map,
    pub humidity_to_location: Map,
}

impl Almanac {
    pub fn suggest_splits(&self, range: (usize, usize)) -> Vec<(usize, usize)> {
        let soil_splits = self.seed_to_soil.suggest_splits(range);
        println!("soilSplits: {:?}", soil_splits);
        let fertilizer_splits = soil_splits
            .iter()
            .flat_map(|split| {
                self.soil_to_fertilizer
                    .suggest_splits((
                        self.map_seed_to_fertilizer(split.0),
                        self.map_seed_to_fertilizer(split.1),
                    ))
                    .iter()
                    .map(|split| {
                        (
                            self.map_fertilizer_to_seed(split.0),
                            self.map_fertilizer_to_seed(split.1),
                        )
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("fertilizerSplits: {:?}", fertilizer_splits);
        let water_splits = fertilizer_splits
            .iter()
            .flat_map(|split| {
                self.fertilizer_to_water
                    .suggest_splits((self.map_seed_to_water(split.0), self.map_seed_to_water(split.1)))
                    .iter()
                    .map(|split| (self.map_water_to_seed(split.0), self.map_water_to_seed(split.1)))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("waterSplits: {:?}", water_splits);
        let light_splits = water_splits
            .iter()
            .flat_map(|split| {
                self.water_to_light
                    .suggest_splits((self.map_seed_to_light(split.0), self.map_seed_to_light(split.1)))
                    .iter()
                    .map(|split| (self.map_light_to_seed(split.0), self.map_light_to_seed(split.1)))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("lightSplits: {:?}", light_splits);
        let temperature_splits = light_splits
            .iter()
            .flat_map(|split| {
                self.light_to_temperature
                    .suggest_splits((
                        self.map_seed_to_temperature(split.0),
                        self.map_seed_to_temperature(split.1),
                    ))
                    .iter()
                    .map(|split| {
                        (
                            self.map_temperature_to_seed(split.0),
                            self.map_temperature_to_seed(split.1),
                        )
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("temperatureSplits: {:?}", temperature_splits);
        let humidity_splits = temperature_splits
            .iter()
            .flat_map(|split| {
                self.temperature_to_humidity
                    .suggest_splits((
                        self.map_seed_to_humidity(split.0),
                        self.map_seed_to_humidity(split.1),
                    ))
                    .iter()
                    .map(|split| {
                        (
                            self.map_humidity_to_seed(split.0),
                            self.map_humidity_to_seed(split.1),
                        )
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("humiditySplits: {:?}", humidity_splits);
        let location_splits = humidity_splits
            .iter()
            .flat_map(|split| {
                self.humidity_to_location
                    .suggest_splits((
                        self.map_seed_to_location(split.0),
                        self.map_seed_to_location(split.1),
                    ))
                    .iter()
                    .map(|split| {
                        (
                            self.map_location_to_seed(split.0),
                            self.map_location_to_seed(split.1),
                        )
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("locationSplits: {:?}", location_splits);
        location_splits
    }

    pub fn map_seed_to_soil(&self, seed: usize) -> usize {
        self.seed_to_soil.map(seed)
    }

    pub fn map_seed_to_fertilizer(&self, seed: usize) -> usize {
        self.soil_to_fertilizer.map(self.map_seed_to_soil(seed))
    }

    pub fn map_seed_to_water(&self, seed: usize) -> usize {
        self.fertilizer_to_water.map(self.map_seed_to_fertilizer(seed))
    }

    pub fn map_seed_to_light(&self, seed: usize) -> usize {
        self.water_to_light.map(self.map_seed_to_water(seed))
    }

    pub fn map_seed_to_temperature(&self, seed: usize) -> usize {
        self.light_to_temperature.map(self.map_seed_to_light(seed))
    }

    pub fn map_seed_to_humidity(&self, seed: usize) -> usize {
        self.temperature_to_humidity
            .map(self.map_seed_to_temperature(seed))
    }

    pub fn map_seed_to_location(&self, seed: usize) -> usize {
        self.humidity_to_location.map(self.map_seed_to_humidity(seed))
    }

    pub fn map_location_to_seed(&self, location: usize) -> usize {
        self.map_humidity_to_seed(self.humidity_to_location.unmap(location))
    }

    pub fn map_humidity_to_seed(&self, humidity: usize) -> usize {
        self.map_temperature_to_seed(self.temperature_to_humidity.unmap(humidity))
    }

    pub fn map_temperature_to_seed(&self, temperature: usize) -> usize {
        self.map_light_to_seed(self.light_to_temperature.unmap(temperature))
    }

    pub fn map_light_to_seed(&self, light: usize) -> usize {
        self.map_water_to_seed(self.water_to_light.unmap(light))
    }

    pub fn map_water_to_seed(&self, water: usize) -> usize {
        self.map_fertilizer_to_seed(self.fertilizer_to_water.unmap(water))
    }

    pub fn map_fertilizer_to_seed(&self, fertilizer: usize) -> usize {
        self.map_soil_to_seed(self.soil_to_fertilizer.unmap(fertilizer))
    }

    pub fn map_soil_to_seed(&self, soil: usize) -> usize {
        self.seed_to_soil.unmap(soil)
    }
}

#[derive(Debug, Clone)]
struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    pub fn map(&self, index: usize) -> usize {
        for mapping in &self.mappings {
            if mapping.in_source(index) {
                return mapping.map(index);
            }
        }
        index
    }

    pub fn unmap(&self, index: usize) -> usize {
        for mapping in &self.mappings {
            if mapping.in_dest(index) {
                return mapping.unmap(index);
            }
        }
        index
    }

    pub fn suggest_splits(&self, range: (usize, usize)) -> Vec<(usize, usize)> {
        let mut touching_mappings = self
            .mappings
            .iter()
            .filter(|m| m.touches_range(range))
            .collect::<Vec<&Mapping>>();
        if touching_mappings.len() == 0 {
            return vec![range];
        }
        touching_mappings.sort_by(|a, b| a.source.cmp(&b.source));
        let mut result = Vec::new();
        let mut next_start = range.0;

        for mapping in &touching_mappings {
            let first_range = mapping.source_range();
            // First Mapping starts after range start
            if first_range.0 > next_start + 1 {
                println!("# fill from {} to {}", next_start, first_range.0 - 1);
                result.push((next_start, first_range.0 - 1));
                next_start = first_range.0;
            }
            let this_end = first_range.1.min(range.1);
            println!(
                "# map from {} to {}",
                first_range.0.max(next_start),
                this_end
            );
            result.push((first_range.0.max(next_start), this_end));
            next_start = this_end + 1;
        }
        if next_start < range.1 {
            result.push((next_start, range.1));
        }
        result
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    source: usize,
    dest: usize,
    length: usize,
}

impl Mapping {
    pub fn touches_range(&self, range: (usize, usize)) -> bool {
        range.0 < self.source + self.length && range.1 >= self.source
    }

    pub fn in_source(&self, index: usize) -> bool {
        index >= self.source && index < self.source + self.length
    }

    pub fn in_dest(&self, index: usize) -> bool {
        index >= self.dest && index < self.dest + self.length
    }

    pub fn source_range(&self) -> (usize, usize) {
        (self.source, self.source + self.length - 1)
    }

    pub fn _dest_range(&self) -> (usize, usize) {
        (self.dest, self.dest + self.length - 1)
    }

    pub fn map(&self, index: usize) -> usize {
        self.dest + (index - self.source)
    }

    pub fn unmap(&self, index: usize) -> usize {
        self.source + (index - self.dest)
    }
}
