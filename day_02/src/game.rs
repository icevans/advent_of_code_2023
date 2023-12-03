use crate::cube_set::CubeSet;
use regex::Regex;

const GAME_DESCRIPTION_PATTERN: &str = r"^Game ([0-9]+): (.+)$";

#[derive(PartialEq, Debug)]
pub struct Game {
    pub id: usize,
    cube_samples: Vec<CubeSet>,
}

impl Game {
    pub fn from(game_description: &str) -> Game {
        let game_regex = Regex::new(GAME_DESCRIPTION_PATTERN).unwrap();
        let (_, [game_id, cube_set_descriptions]) = game_regex
            .captures(game_description)
            .unwrap()
            .extract::<2>();

        Game {
            id: game_id.parse().expect("This should be a number string, it's from a regex matching digits!"),
            cube_samples: cube_set_descriptions
                .split("; ")
                .map(|cube_set_description| CubeSet::from(cube_set_description))
                .collect(),
        }
    }

    pub fn is_possible_for_cube_set(&self, total_cube_set: &CubeSet) -> bool {
        self.cube_samples
            .iter()
            .all(|cube_sample| cube_sample.contained_in(&total_cube_set))
    }

    pub fn minimum_starting_cube_set(&self) -> CubeSet {
        CubeSet {
            red: self.max_red(),
            blue: self.max_blue(),
            green: self.max_green(),
        }
    }

    fn max_red(&self) -> usize {
        self.cube_samples.iter().fold(0, |curr_largest, cube_set| {
            if cube_set.red > curr_largest {
                cube_set.red
            } else {
                curr_largest
            }
        })
    }

    fn max_blue(&self) -> usize {
        self.cube_samples.iter().fold(0, |curr_largest, cube_set| {
            if cube_set.blue > curr_largest {
                cube_set.blue
            } else {
                curr_largest
            }
        })
    }

    fn max_green(&self) -> usize {
        self.cube_samples.iter().fold(0, |curr_largest, cube_set| {
            if cube_set.green > curr_largest {
                cube_set.green
            } else {
                curr_largest
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::cube_set::CubeSet;
    use crate::game::Game;

    #[test]
    fn possible_game() {
        let total_cube_set = CubeSet {
            red: 4,
            blue: 3,
            green: 6,
        };

        let game_cube_sample_1 = CubeSet {
            red: 2,
            blue: 1,
            green: 0,
        };

        let game_cube_sample_2 = CubeSet {
            red: 4,
            blue: 0,
            green: 6,
        };

        let game = Game {
            id: 0,
            cube_samples: vec![game_cube_sample_1, game_cube_sample_2],
        };

        assert_eq!(true, game.is_possible_for_cube_set(&total_cube_set))
    }

    #[test]
    fn impossible_game() {
        let total_cube_set = CubeSet {
            red: 1,
            blue: 1,
            green: 1,
        };

        let game_cube_sample_1 = CubeSet {
            red: 2,
            blue: 1,
            green: 0,
        };

        let game_cube_sample_2 = CubeSet {
            red: 4,
            blue: 0,
            green: 6,
        };

        let game = Game {
            id: 0,
            cube_samples: vec![game_cube_sample_1, game_cube_sample_2],
        };

        assert_eq!(false, game.is_possible_for_cube_set(&total_cube_set))
    }

    #[test]
    fn game_description_1() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            Game {
                id: 1,
                cube_samples: vec![
                    CubeSet {
                        blue: 3,
                        red: 4,
                        green: 0,
                    },
                    CubeSet {
                        blue: 6,
                        red: 1,
                        green: 2,
                    },
                    CubeSet {
                        blue: 0,
                        red: 0,
                        green: 2,
                    }
                ]
            },
            game
        );
    }

    #[test]
    fn game_description_2() {
        let game = Game::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");

        assert_eq!(
            Game {
                id: 2,
                cube_samples: vec![
                    CubeSet {
                        blue: 1,
                        red: 0,
                        green: 2,
                    },
                    CubeSet {
                        blue: 4,
                        red: 1,
                        green: 3,
                    },
                    CubeSet {
                        blue: 1,
                        red: 0,
                        green: 1,
                    }
                ]
            },
            game
        );
    }

    #[test]
    fn game_description_3() {
        let game =
            Game::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");

        assert_eq!(
            Game {
                id: 3,
                cube_samples: vec![
                    CubeSet {
                        blue: 6,
                        red: 20,
                        green: 8,
                    },
                    CubeSet {
                        blue: 5,
                        red: 4,
                        green: 13,
                    },
                    CubeSet {
                        blue: 0,
                        red: 1,
                        green: 5,
                    }
                ]
            },
            game
        );
    }

    #[test]
    fn game_description_4() {
        let game =
            Game::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");

        assert_eq!(
            Game {
                id: 4,
                cube_samples: vec![
                    CubeSet {
                        blue: 6,
                        red: 3,
                        green: 1,
                    },
                    CubeSet {
                        blue: 0,
                        red: 6,
                        green: 3,
                    },
                    CubeSet {
                        blue: 15,
                        red: 14,
                        green: 3,
                    }
                ]
            },
            game
        );
    }

    #[test]
    fn game_description_5() {
        let game = Game::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        assert_eq!(
            Game {
                id: 5,
                cube_samples: vec![
                    CubeSet {
                        blue: 1,
                        red: 6,
                        green: 3,
                    },
                    CubeSet {
                        blue: 2,
                        red: 1,
                        green: 2,
                    },
                ]
            },
            game
        );
    }

    #[test]
    fn minimum_cube_set_for_game_1() {
        let game = Game::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(
            CubeSet {
                red: 4,
                green: 2,
                blue: 6
            },
            game.minimum_starting_cube_set()
        );
        assert_eq!(48, game.minimum_starting_cube_set().power());
    }

    #[test]
    fn minimum_cube_set_for_game_2() {
        let game = Game::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");
        assert_eq!(
            CubeSet {
                red: 1,
                green: 3,
                blue: 4
            },
            game.minimum_starting_cube_set()
        );
        assert_eq!(12, game.minimum_starting_cube_set().power());
    }

    #[test]
    fn minimum_cube_set_for_game_3() {
        let game =
            Game::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
        assert_eq!(
            CubeSet {
                red: 20,
                green: 13,
                blue: 6
            },
            game.minimum_starting_cube_set()
        );
        assert_eq!(1560, game.minimum_starting_cube_set().power());
    }

    #[test]
    fn minimum_cube_set_for_game_4() {
        let game =
            Game::from("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red");
        assert_eq!(
            CubeSet {
                red: 14,
                green: 3,
                blue: 15
            },
            game.minimum_starting_cube_set()
        );
        assert_eq!(630, game.minimum_starting_cube_set().power());
    }

    #[test]
    fn minimum_cube_set_for_game_5() {
        let game = Game::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(
            CubeSet {
                red: 6,
                green: 3,
                blue: 2
            },
            game.minimum_starting_cube_set()
        );
        assert_eq!(36, game.minimum_starting_cube_set().power());
    }
}
