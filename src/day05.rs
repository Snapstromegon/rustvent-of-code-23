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
        let mappings: Vec<(&str, Map)> = blocks.map(|block| parseMapping(block)).collect();
        let alm = Almanac {
            seedToSoil: mappings[0].1.clone(),
            soilToFertilizer: mappings[1].1.clone(),
            fertilizerToWater: mappings[2].1.clone(),
            waterToLight: mappings[3].1.clone(),
            lightToTemperature: mappings[4].1.clone(),
            temperatureToHumidity: mappings[5].1.clone(),
            humidityToLocation: mappings[6].1.clone(),
        };
        let mut sorted = seeds
            .iter()
            .map(|seed| alm.mapSeedToLocation(*seed))
            .collect::<Vec<usize>>();
        sorted.sort();

        Some(sorted[0])
    }

    fn part2(&self, input: &str) -> Option<usize> {
        let mut blocks = input.split("\r\n\r\n");
        let seedPairs = blocks
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
        let mappings: Vec<(&str, Map)> = blocks.map(|block| parseMapping(block)).collect();
        let alm = Almanac {
            seedToSoil: mappings[0].1.clone(),
            soilToFertilizer: mappings[1].1.clone(),
            fertilizerToWater: mappings[2].1.clone(),
            waterToLight: mappings[3].1.clone(),
            lightToTemperature: mappings[4].1.clone(),
            temperatureToHumidity: mappings[5].1.clone(),
            humidityToLocation: mappings[6].1.clone(),
        };
        for i in 0..100 {
            assert!(alm.seedToSoil.map(i) == alm.mapSeedToSoil(i));
            assert!(alm.seedToSoil.unmap(i) == alm.mapSoilToSeed(i));
            assert!(alm.soilToFertilizer.map(alm.seedToSoil.map(i)) == alm.mapSeedToFertilizer(i));
            assert!(alm.seedToSoil.unmap(alm.soilToFertilizer.unmap(i)) == alm.mapFertilizerToSeed(i));
            assert!(alm.fertilizerToWater.map(alm.soilToFertilizer.map(alm.seedToSoil.map(i))) == alm.mapSeedToWater(i));
            assert!(alm.seedToSoil.unmap(alm.soilToFertilizer.unmap(alm.fertilizerToWater.unmap(i))) == alm.mapWaterToSeed(i));
            assert!(alm.waterToLight.map(alm.fertilizerToWater.map(alm.soilToFertilizer.map(alm.seedToSoil.map(i)))) == alm.mapSeedToLight(i));
            assert!(alm.seedToSoil.unmap(alm.soilToFertilizer.unmap(alm.fertilizerToWater.unmap(alm.waterToLight.unmap(i)))) == alm.mapLightToSeed(i));
            assert!(alm.lightToTemperature.map(alm.waterToLight.map(alm.fertilizerToWater.map(alm.soilToFertilizer.map(alm.seedToSoil.map(i))))) == alm.mapSeedToTemperature(i));
            assert!(alm.seedToSoil.unmap(alm.soilToFertilizer.unmap(alm.fertilizerToWater.unmap(alm.waterToLight.unmap(alm.lightToTemperature.unmap(i))))) == alm.mapTemperatureToSeed(i));
            assert!(alm.temperatureToHumidity.map(alm.lightToTemperature.map(alm.waterToLight.map(alm.fertilizerToWater.map(alm.soilToFertilizer.map(alm.seedToSoil.map(i)))))) == alm.mapSeedToHumidity(i));
            assert!(alm.seedToSoil.unmap(alm.soilToFertilizer.unmap(alm.fertilizerToWater.unmap(alm.waterToLight.unmap(alm.lightToTemperature.unmap(alm.temperatureToHumidity.unmap(i)))))) == alm.mapHumidityToSeed(i));
            assert!(alm.humidityToLocation.map(alm.temperatureToHumidity.map(alm.lightToTemperature.map(alm.waterToLight.map(alm.fertilizerToWater.map(alm.soilToFertilizer.map(alm.seedToSoil.map(i))))))) == alm.mapSeedToLocation(i));
            assert!(alm.seedToSoil.unmap(alm.soilToFertilizer.unmap(alm.fertilizerToWater.unmap(alm.waterToLight.unmap(alm.lightToTemperature.unmap(alm.temperatureToHumidity.unmap(alm.humidityToLocation.unmap(i))))))) == alm.mapLocationToSeed(i));
        }



        for seedPair in seedPairs {
            println!(
                "pair: {:?}, splits: {:?}",
                seedPair,
                alm.suggestSplits(seedPair)
            );
        }
        None
    }
}

fn parseMapping(block: &str) -> (&str, Map) {
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
    pub seedToSoil: Map,
    pub soilToFertilizer: Map,
    pub fertilizerToWater: Map,
    pub waterToLight: Map,
    pub lightToTemperature: Map,
    pub temperatureToHumidity: Map,
    pub humidityToLocation: Map,
}

impl Almanac {
    pub fn suggestSplits(&self, range: (usize, usize)) -> Vec<(usize, usize)> {
        let soilSplits = self.seedToSoil.suggestSplits(range);
        println!("soilSplits: {:?}", soilSplits);
        let fertilizerSplits = soilSplits
            .iter()
            .flat_map(|split| {
                self.soilToFertilizer
                    .suggestSplits((
                        self.mapSeedToFertilizer(split.0),
                        self.mapSeedToFertilizer(split.1),
                    ))
                    .iter()
                    .map(|split| {
                        (
                            self.mapFertilizerToSeed(split.0),
                            self.mapFertilizerToSeed(split.1),
                        )
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("fertilizerSplits: {:?}", fertilizerSplits);
        let waterSplits = fertilizerSplits
            .iter()
            .flat_map(|split| {
                self.fertilizerToWater
                    .suggestSplits((self.mapSeedToWater(split.0), self.mapSeedToWater(split.1)))
                    .iter()
                    .map(|split| (self.mapWaterToSeed(split.0), self.mapWaterToSeed(split.1)))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("waterSplits: {:?}", waterSplits);
        let lightSplits = waterSplits
            .iter()
            .flat_map(|split| {
                self.waterToLight
                    .suggestSplits((self.mapSeedToLight(split.0), self.mapSeedToLight(split.1)))
                    .iter()
                    .map(|split| (self.mapLightToSeed(split.0), self.mapLightToSeed(split.1)))
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("lightSplits: {:?}", lightSplits);
        let temperatureSplits = lightSplits
            .iter()
            .flat_map(|split| {
                self.lightToTemperature
                    .suggestSplits((
                        self.mapSeedToTemperature(split.0),
                        self.mapSeedToTemperature(split.1),
                    ))
                    .iter()
                    .map(|split| {
                        (
                            self.mapTemperatureToSeed(split.0),
                            self.mapTemperatureToSeed(split.1),
                        )
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("temperatureSplits: {:?}", temperatureSplits);
        let humiditySplits = temperatureSplits
            .iter()
            .flat_map(|split| {
                self.temperatureToHumidity
                    .suggestSplits((
                        self.mapSeedToHumidity(split.0),
                        self.mapSeedToHumidity(split.1),
                    ))
                    .iter()
                    .map(|split| {
                        (
                            self.mapHumidityToSeed(split.0),
                            self.mapHumidityToSeed(split.1),
                        )
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("humiditySplits: {:?}", humiditySplits);
        let locationSplits = humiditySplits
            .iter()
            .flat_map(|split| {
                self.humidityToLocation
                    .suggestSplits((
                        self.mapSeedToLocation(split.0),
                        self.mapSeedToLocation(split.1),
                    ))
                    .iter()
                    .map(|split| {
                        (
                            self.mapLocationToSeed(split.0),
                            self.mapLocationToSeed(split.1),
                        )
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<Vec<(usize, usize)>>();
        println!("locationSplits: {:?}", locationSplits);
        locationSplits
    }

    pub fn mapSeedToSoil(&self, seed: usize) -> usize {
        self.seedToSoil.map(seed)
    }

    pub fn mapSeedToFertilizer(&self, seed: usize) -> usize {
        self.soilToFertilizer.map(self.mapSeedToSoil(seed))
    }

    pub fn mapSeedToWater(&self, seed: usize) -> usize {
        self.fertilizerToWater.map(self.mapSeedToFertilizer(seed))
    }

    pub fn mapSeedToLight(&self, seed: usize) -> usize {
        self.waterToLight.map(self.mapSeedToWater(seed))
    }

    pub fn mapSeedToTemperature(&self, seed: usize) -> usize {
        self.lightToTemperature.map(self.mapSeedToLight(seed))
    }

    pub fn mapSeedToHumidity(&self, seed: usize) -> usize {
        self.temperatureToHumidity
            .map(self.mapSeedToTemperature(seed))
    }

    pub fn mapSeedToLocation(&self, seed: usize) -> usize {
        self.humidityToLocation.map(self.mapSeedToHumidity(seed))
    }

    pub fn mapLocationToSeed(&self, location: usize) -> usize {
        self.mapHumidityToSeed(self.humidityToLocation.unmap(location))
    }

    pub fn mapHumidityToSeed(&self, humidity: usize) -> usize {
        self.mapTemperatureToSeed(self.temperatureToHumidity.unmap(humidity))
    }

    pub fn mapTemperatureToSeed(&self, temperature: usize) -> usize {
        self.mapLightToSeed(self.lightToTemperature.unmap(temperature))
    }

    pub fn mapLightToSeed(&self, light: usize) -> usize {
        self.mapWaterToSeed(self.waterToLight.unmap(light))
    }

    pub fn mapWaterToSeed(&self, water: usize) -> usize {
        self.mapFertilizerToSeed(self.fertilizerToWater.unmap(water))
    }

    pub fn mapFertilizerToSeed(&self, fertilizer: usize) -> usize {
        self.mapSoilToSeed(self.soilToFertilizer.unmap(fertilizer))
    }

    pub fn mapSoilToSeed(&self, soil: usize) -> usize {
        self.seedToSoil.unmap(soil)
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

    pub fn suggestSplits(&self, range: (usize, usize)) -> Vec<(usize, usize)> {
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

    pub fn dest_range(&self) -> (usize, usize) {
        (self.dest, self.dest + self.length - 1)
    }

    pub fn map(&self, index: usize) -> usize {
        self.dest + (index - self.source)
    }

    pub fn unmap(&self, index: usize) -> usize {
        self.source + (index - self.dest)
    }
}
