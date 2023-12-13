use crate::solution::Solution;

pub struct Day;

impl Solution for Day {
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day05::Day;
    /// let input = read_input(5, true, 1).unwrap();
    /// assert_eq!(Day.part1(&input), Some(35))
    /// ```
    ///
    /// ```
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day05::Day;
    /// let input = read_input(5, false, 1).unwrap();
    /// assert_eq!(Day.part1(&input), Some(51752125))
    /// ```
    fn part1(&self, input: &str) -> Option<usize> {
        let mut blocks = input.split("\r\n\r\n").flat_map(|s| s.split("\n\n"));
        let seeds = blocks
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let mappings: Vec<(&str, Map)> = blocks.map(parse_mapping).collect();
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

    /// ```ignore
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day05::Day;
    /// let input = read_input(5, true, 2).unwrap();
    /// assert_eq!(Day.part2(&input), Some(46))
    /// ```
    ///
    /// ```ignore
    /// # use rustvent::utils::read_input;
    /// # use rustvent::solution::Solution;
    /// # use rustvent::day05::Day;
    /// let input = read_input(5, false, 2).unwrap();
    /// assert_eq!(Day.part2(&input), Some(12634632))
    /// ```
    fn part2(&self, _input: &str) -> Option<usize> {
        None
    }
}

fn parse_mapping(block: &str) -> (&str, Map) {
    let (name, rest) = block.split_once(':').unwrap();
    let mappings = rest
        .trim()
        .lines()
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
    pub fn map_seed_to_soil(&self, seed: usize) -> usize {
        self.seed_to_soil.map(seed)
    }

    pub fn map_seed_to_fertilizer(&self, seed: usize) -> usize {
        self.soil_to_fertilizer.map(self.map_seed_to_soil(seed))
    }

    pub fn map_seed_to_water(&self, seed: usize) -> usize {
        self.fertilizer_to_water
            .map(self.map_seed_to_fertilizer(seed))
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
        self.humidity_to_location
            .map(self.map_seed_to_humidity(seed))
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
}

#[derive(Debug, Clone)]
struct Mapping {
    source: usize,
    dest: usize,
    length: usize,
}

impl Mapping {
    pub fn in_source(&self, index: usize) -> bool {
        index >= self.source && index < self.source + self.length
    }

    pub fn _dest_range(&self) -> (usize, usize) {
        (self.dest, self.dest + self.length - 1)
    }

    pub fn map(&self, index: usize) -> usize {
        self.dest + (index - self.source)
    }
}
