use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

struct AlmanacMap {
    range_maps: Vec<RangeMap>,
}

impl AlmanacMap {
    pub fn new(range_maps: Vec<RangeMap>) -> AlmanacMap {
        AlmanacMap { range_maps }
    }

    pub fn get(&self, k: u64) -> u64 {
        if let Some(range_map) = self.get_range_map_for_key(k) {
            range_map.get(k).expect("this range map covers this key")
        } else {
            k
        }
    }

    fn get_range_map_for_key(&self, k: u64) -> Option<&RangeMap> {
        let appropriate_range_maps: Vec<&RangeMap> = self
            .range_maps
            .iter()
            .filter(|range_map| {
                k >= range_map.source_start && k - range_map.source_start < range_map.range
            })
            .collect();

        if appropriate_range_maps.len() == 0 {
            return None;
        }

        if appropriate_range_maps.len() > 1 {
            panic!("overlapping ranges!")
        }

        Some(&appropriate_range_maps[0])
    }
}

#[cfg(test)]
mod almanac_map_tests {
    use crate::{AlmanacMap, RangeMap};

    #[test]
    fn test_source_start() {
        let map = AlmanacMap::new(vec![RangeMap::new(98, 50, 2), RangeMap::new(50, 52, 48)]);

        assert_eq!(50, map.get(98))
    }

    #[test]
    fn test_middle() {
        let map = AlmanacMap::new(vec![RangeMap::new(98, 50, 2), RangeMap::new(50, 52, 48)]);

        assert_eq!(53, map.get(51))
    }

    #[test]
    fn test_source_end() {
        let map = AlmanacMap::new(vec![RangeMap::new(98, 50, 2), RangeMap::new(50, 52, 48)]);

        assert_eq!(51, map.get(99))
    }

    #[test]
    fn test_past_end() {
        let map = AlmanacMap::new(vec![RangeMap::new(98, 50, 2), RangeMap::new(50, 52, 48)]);

        assert_eq!(100, map.get(100))
    }

    #[test]
    fn test_before_source_start() {
        let map = AlmanacMap::new(vec![RangeMap::new(98, 50, 2), RangeMap::new(50, 52, 48)]);

        assert_eq!(49, map.get(49))
    }
}

struct RangeMap {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

impl RangeMap {
    fn new(source_start: u64, destination_start: u64, range: u64) -> RangeMap {
        RangeMap {
            source_start,
            destination_start,
            range,
        }
    }

    fn get(&self, k: u64) -> Option<u64> {
        if k < self.source_start {
            return None;
        }

        let difference = k - self.source_start;

        if difference >= self.range {
            return None;
        }

        Some(self.destination_start + difference)
    }
}

#[cfg(test)]
mod range_map_tests {
    use crate::RangeMap;

    #[test]
    fn test_source_start() {
        let map = RangeMap::new(98, 50, 2);

        assert_eq!(Some(50), map.get(98))
    }

    #[test]
    fn test_source_end() {
        let map = RangeMap::new(98, 50, 2);

        assert_eq!(Some(51), map.get(99))
    }

    #[test]
    fn test_past_end() {
        let map = RangeMap::new(98, 50, 2);

        assert_eq!(None, map.get(100))
    }

    #[test]
    fn test_before_source_start() {
        let map = RangeMap::new(98, 50, 2);

        assert_eq!(None, map.get(97))
    }
}

fn main() {
    let mut input = include_str!("../input.txt").lines().peekable();

    let inital_seed_regex = Regex::new(r"^seeds: (.+)$").expect("valid regex");
    // TODO: Make this more general to match the other maps and extract their names
    let seed_to_soil_regex = Regex::new(r"^seed-to-soil map:$").expect("valid regex");
    let soil_to_fertilizer = Regex::new(r"^soil-to-fertilizer map:$").expect("valid regex");
    let fertilizer_to_water = Regex::new(r"^fertilizer-to-water map:$").expect("valid regex");
    let water_to_light = Regex::new(r"^water-to-light map:$").expect("valid regex");
    let light_to_temperature = Regex::new(r"^light-to-temperature map:$").expect("valid regex");
    let temperature_to_humidity =
        Regex::new(r"^temperature-to-humidity map:$").expect("valid regex");
    let humidity_to_location = Regex::new(r"^humidity-to-location map:$").expect("valid regex");
    let map_data_regex = Regex::new(r"^([0-9]+) +([0-9]+) +([0-9]+)$").expect("valid regex");

    let mut seed_to_soil_map = AlmanacMap::new(vec![]);
    let mut soil_to_fertilizer_map = AlmanacMap::new(vec![]);
    let mut fertilizer_to_water_map = AlmanacMap::new(vec![]);
    let mut water_to_light_map = AlmanacMap::new(vec![]);
    let mut light_to_temperature_map = AlmanacMap::new(vec![]);
    let mut temperature_to_humidity_map = AlmanacMap::new(vec![]);
    let mut humidity_to_location_map = AlmanacMap::new(vec![]);

    let mut initial_seeds: Vec<u64> = vec![];

    while let Some(line) = input.next() {
        if inital_seed_regex.is_match(line) {
            let (_, [seed_numbers]) = inital_seed_regex.captures(line).unwrap().extract();
            for num_string in seed_numbers.split_whitespace() {
                initial_seeds.push(num_string.parse().unwrap());
            }
        }

        if seed_to_soil_regex.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                seed_to_soil_map.range_maps.push(RangeMap::new(
                    src_range_start,
                    dest_range_start,
                    range_length,
                ));
            }
        }

        if soil_to_fertilizer.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                soil_to_fertilizer_map.range_maps.push(RangeMap::new(
                    src_range_start,
                    dest_range_start,
                    range_length,
                ));
            }
        }

        if fertilizer_to_water.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                fertilizer_to_water_map.range_maps.push(RangeMap::new(
                    src_range_start,
                    dest_range_start,
                    range_length,
                ));
            }
        }

        if water_to_light.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                water_to_light_map.range_maps.push(RangeMap::new(
                    src_range_start,
                    dest_range_start,
                    range_length,
                ));
            }
        }

        if light_to_temperature.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                light_to_temperature_map.range_maps.push(RangeMap::new(
                    src_range_start,
                    dest_range_start,
                    range_length,
                ));
            }
        }

        if temperature_to_humidity.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                temperature_to_humidity_map.range_maps.push(RangeMap::new(
                    src_range_start,
                    dest_range_start,
                    range_length,
                ));
            }
        }

        if humidity_to_location.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                humidity_to_location_map.range_maps.push(RangeMap::new(
                    src_range_start,
                    dest_range_start,
                    range_length,
                ));
            }
        }
    }

    let mut seed_with_lowest_location = initial_seeds[0];
    let mut lowest_location: u64 = 999_999_999_999;

    for seed in initial_seeds {
        let soil = seed_to_soil_map.get(seed);
        let fertilizer = soil_to_fertilizer_map.get(soil);
        let water = fertilizer_to_water_map.get(fertilizer);
        let light = water_to_light_map.get(water);
        let temperature = light_to_temperature_map.get(light);
        let humidity = temperature_to_humidity_map.get(temperature);
        let location = humidity_to_location_map.get(humidity);

        if location < lowest_location {
            seed_with_lowest_location = seed;
            lowest_location = location;
        }
    }

    println!("Lowest seed: {seed_with_lowest_location} (location {lowest_location})");
}
