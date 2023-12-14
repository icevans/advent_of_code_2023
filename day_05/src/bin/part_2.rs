use rangemap::RangeMap;
use regex::Regex;
use std::ops::Range;

struct AlmanacMap {
    range_map: RangeMap<u64, RangeSpecifiedHashMap>,
}

impl AlmanacMap {
    pub fn new(range_maps: Vec<RangeSpecifiedHashMap>) -> AlmanacMap {
        let mut almanac = AlmanacMap {
            range_map: RangeMap::new(),
        };
        for map in range_maps {
            almanac
                .range_map
                .insert(map.source_start..map.source_start + map.range, map);
        }

        almanac
    }

    pub fn get(&self, k: u64) -> u64 {
        if let Some(map) = self.range_map.get(&k) {
            map.get(k).unwrap()
        } else {
            k
        }
    }
}

#[cfg(test)]
mod almanac_map_tests {
    use crate::{AlmanacMap, RangeSpecifiedHashMap};

    #[test]
    fn test_source_start() {
        let map = AlmanacMap::new(vec![
            RangeSpecifiedHashMap::new(98, 50, 2),
            RangeSpecifiedHashMap::new(50, 52, 48),
        ]);

        assert_eq!(50, map.get(98))
    }

    #[test]
    fn test_middle() {
        let map = AlmanacMap::new(vec![
            RangeSpecifiedHashMap::new(98, 50, 2),
            RangeSpecifiedHashMap::new(50, 52, 48),
        ]);

        assert_eq!(53, map.get(51))
    }

    #[test]
    fn test_source_end() {
        let map = AlmanacMap::new(vec![
            RangeSpecifiedHashMap::new(98, 50, 2),
            RangeSpecifiedHashMap::new(50, 52, 48),
        ]);

        assert_eq!(51, map.get(99))
    }

    #[test]
    fn test_past_end() {
        let map = AlmanacMap::new(vec![
            RangeSpecifiedHashMap::new(98, 50, 2),
            RangeSpecifiedHashMap::new(50, 52, 48),
        ]);

        assert_eq!(100, map.get(100))
    }

    #[test]
    fn test_before_source_start() {
        let map = AlmanacMap::new(vec![
            RangeSpecifiedHashMap::new(98, 50, 2),
            RangeSpecifiedHashMap::new(50, 52, 48),
        ]);

        assert_eq!(49, map.get(49))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct RangeSpecifiedHashMap {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

impl RangeSpecifiedHashMap {
    fn new(source_start: u64, destination_start: u64, range: u64) -> RangeSpecifiedHashMap {
        RangeSpecifiedHashMap {
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
    use crate::RangeSpecifiedHashMap;

    #[test]
    fn test_source_start() {
        let map = RangeSpecifiedHashMap::new(98, 50, 2);

        assert_eq!(Some(50), map.get(98))
    }

    #[test]
    fn test_source_end() {
        let map = RangeSpecifiedHashMap::new(98, 50, 2);

        assert_eq!(Some(51), map.get(99))
    }

    #[test]
    fn test_past_end() {
        let map = RangeSpecifiedHashMap::new(98, 50, 2);

        assert_eq!(None, map.get(100))
    }

    #[test]
    fn test_before_source_start() {
        let map = RangeSpecifiedHashMap::new(98, 50, 2);

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

    let mut initial_seed_ranges: Vec<Range<u64>> = vec![];
    let start = std::time::Instant::now();
    while let Some(line) = input.next() {
        if inital_seed_regex.is_match(line) {
            let (_, [seed_numbers]) = inital_seed_regex.captures(line).unwrap().extract();
            let seed_numbers: Vec<&str> = seed_numbers.split_whitespace().collect();

            let mut i = 0;
            while i < seed_numbers.len() - 2 {
                let range_start: u64 = seed_numbers[i].parse().unwrap();
                let range_end: u64 = range_start + seed_numbers[i + 1].parse::<u64>().unwrap();
                initial_seed_ranges.push(range_start..range_end);
                i += 2;
            }
        }

        if seed_to_soil_regex.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                seed_to_soil_map.range_map.insert(
                    src_range_start..src_range_start + range_length,
                    RangeSpecifiedHashMap::new(src_range_start, dest_range_start, range_length),
                );
            }
        }

        if soil_to_fertilizer.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                soil_to_fertilizer_map.range_map.insert(
                    src_range_start..src_range_start + range_length,
                    RangeSpecifiedHashMap::new(src_range_start, dest_range_start, range_length),
                );
            }
        }

        if fertilizer_to_water.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                fertilizer_to_water_map.range_map.insert(
                    src_range_start..src_range_start + range_length,
                    RangeSpecifiedHashMap::new(src_range_start, dest_range_start, range_length),
                );
            }
        }

        if water_to_light.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                water_to_light_map.range_map.insert(
                    src_range_start..src_range_start + range_length,
                    RangeSpecifiedHashMap::new(src_range_start, dest_range_start, range_length),
                );
            }
        }

        if light_to_temperature.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                light_to_temperature_map.range_map.insert(
                    src_range_start..src_range_start + range_length,
                    RangeSpecifiedHashMap::new(src_range_start, dest_range_start, range_length),
                );
            }
        }

        if temperature_to_humidity.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                temperature_to_humidity_map.range_map.insert(
                    src_range_start..src_range_start + range_length,
                    RangeSpecifiedHashMap::new(src_range_start, dest_range_start, range_length),
                );
            }
        }

        if humidity_to_location.is_match(line) {
            while input.peek().unwrap_or(&"") != &"" {
                let data = input.next().unwrap();
                let (_, map_data) = map_data_regex.captures(data).unwrap().extract::<3>();

                let [dest_range_start, src_range_start, range_length] =
                    map_data.map(|n| n.parse::<u64>().unwrap());

                humidity_to_location_map.range_map.insert(
                    src_range_start..src_range_start + range_length,
                    RangeSpecifiedHashMap::new(src_range_start, dest_range_start, range_length),
                );
            }
        }
    }

    let mut lowest_location: u64 = 999_999_999_999;

    for seed_range in initial_seed_ranges {
        for seed in seed_range {
            let soil = seed_to_soil_map.get(seed);
            let fertilizer = soil_to_fertilizer_map.get(soil);
            let water = fertilizer_to_water_map.get(fertilizer);
            let light = water_to_light_map.get(water);
            let temperature = light_to_temperature_map.get(light);
            let humidity = temperature_to_humidity_map.get(temperature);
            let location = humidity_to_location_map.get(humidity);

            if location < lowest_location {
                lowest_location = location;
            }
        }
    }

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
    println!("Lowest seed location: {lowest_location}");
}
