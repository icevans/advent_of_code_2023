#[derive(PartialEq, Debug)]
pub struct CubeSet {
    pub red: usize,
    pub blue: usize,
    pub green: usize,
}

impl CubeSet {
    pub fn from(cube_set_description: &str) -> CubeSet {
        // TODO: Rather than panic, this method should return a Result
        //       and an error in the case that the input is invalid. Callers
        //       can decide what they want to do in that case.
        let mut cube_set = CubeSet {
            red: 0,
            blue: 0,
            green: 0,
        };

        for color_description in cube_set_description.split(",") {
            let parts: Vec<&str> = color_description.trim().split(" ").collect();

            match parts[1] {
                "green" => {
                    cube_set.green = parts[0].parse().unwrap();
                }
                "blue" => {
                    cube_set.blue = parts[0].parse().unwrap();
                }
                "red" => {
                    cube_set.red = parts[0].parse().unwrap();
                }
                _ => {
                    panic!();
                }
            };
        }

        cube_set
    }

    pub fn contained_in(&self, other: &CubeSet) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[cfg(test)]
mod tests {
    use super::CubeSet;

    #[test]
    fn contained_cube_set() {
        let smaller_cube_set = CubeSet {
            red: 1,
            green: 1,
            blue: 2,
        };

        let larger_cube_set = CubeSet {
            red: 2,
            green: 2,
            blue: 2,
        };

        let result = smaller_cube_set.contained_in(&larger_cube_set);
        assert_eq!(true, result);
    }

    #[test]
    fn uncontained_cube_set() {
        let smaller_cube_set = CubeSet {
            red: 1,
            green: 1,
            blue: 2,
        };

        let larger_cube_set = CubeSet {
            red: 2,
            green: 2,
            blue: 2,
        };

        let result = larger_cube_set.contained_in(&smaller_cube_set);
        assert_eq!(false, result);
    }

    #[test]
    fn from_description_1() {
        let result = CubeSet::from("3 blue, 4 red");
        assert_eq!(
            CubeSet {
                red: 4,
                blue: 3,
                green: 0,
            },
            result
        )
    }

    #[test]
    fn from_description_2() {
        let result = CubeSet::from("1 red, 2 green, 6 blue");
        assert_eq!(
            CubeSet {
                red: 1,
                blue: 6,
                green: 2,
            },
            result
        )
    }

    #[test]
    fn from_description_3() {
        let result = CubeSet::from("2 green");
        assert_eq!(
            CubeSet {
                red: 0,
                blue: 0,
                green: 2,
            },
            result
        )
    }
}
