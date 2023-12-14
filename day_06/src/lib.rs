use regex::Regex;

pub fn parse_races(race_descriptions: &str) -> Vec<Race> {
    let time_regex = Regex::new(r"^Time: +(.+)$").expect("regex is valid");
    let distance_regex = Regex::new(r"^Distance: +(.+)$").expect("regex is valid");

    let mut race_times: Vec<u64> = vec![];
    let mut race_durations: Vec<u64> = vec![];

    for line in race_descriptions.lines() {
        if let Some(captures) = time_regex.captures(line) {
            let (_, [times]) = captures.extract();

            let mut times: Vec<u64> = times
                .split_whitespace()
                .map(|t| t.parse().unwrap())
                .collect();
            race_times.append(&mut times);
        }

        if let Some(captures) = distance_regex.captures(line) {
            let (_, [durations]) = captures.extract();
            let mut durations = durations
                .split_whitespace()
                .map(|t| t.parse().unwrap())
                .collect();
            race_durations.append(&mut durations);
        }
    }

    race_times
        .iter()
        .enumerate()
        .map(|(i, time)| Race::new(*time, race_durations[i]))
        .collect()
}

pub fn parse_race_part_2(race_descriptions: &str) -> Race {
    let time_regex = Regex::new(r"^Time: +(.+)$").expect("regex is valid");
    let distance_regex = Regex::new(r"^Distance: +(.+)$").expect("regex is valid");

    let mut race_time: u64 = 0;
    let mut race_duration: u64 = 0;

    for line in race_descriptions.lines() {
        if let Some(captures) = time_regex.captures(line) {
            let (_, [time]) = captures.extract();
            let time: u64 = time.replace(" ", "").parse().unwrap();
            race_time = time;
        }

        if let Some(captures) = distance_regex.captures(line) {
            let (_, [duration]) = captures.extract();
            let duration: u64 = duration.replace(" ", "").parse().unwrap();
            race_duration = duration;
        }
    }

    Race::new(race_time, race_duration)
}

#[derive(PartialEq, Debug)]
pub struct Race {
    duration: u64,
    record_distance: u64,
    pub winning_button_hold_times: Vec<u64>,
}

impl Race {
    pub fn new(duration: u64, record_distance: u64) -> Race {
        let mut race = Race {
            duration,
            record_distance,
            winning_button_hold_times: vec![],
        };

        race.calculate_winning_button_hold_times();
        race
    }

    fn calculate_winning_button_hold_times(&mut self) {
        for button_hold_time in 0..self.duration {
            let distance = (self.duration - button_hold_time) * button_hold_time;
            if distance > self.record_distance {
                self.winning_button_hold_times.push(button_hold_time);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_1() {
        let race = Race::new(7, 9);
        assert_eq!(vec![2, 3, 4, 5], race.winning_button_hold_times);
    }

    #[test]
    fn sample_input_2() {
        let race = Race::new(15, 40);
        assert_eq!(
            vec![4, 5, 6, 7, 8, 9, 10, 11],
            race.winning_button_hold_times
        );
    }

    #[test]
    fn sample_input_3() {
        let race = Race::new(30, 200);
        assert_eq!(
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19],
            race.winning_button_hold_times
        );
    }
}
