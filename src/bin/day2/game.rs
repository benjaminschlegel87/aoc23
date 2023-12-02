use std::ops::Add;

/// Maximum amount of Red Cubes that can be in any samples
const RED_CUBES: usize = 12;
/// Maximum amount of Blue Cubes that can be in any samples
const BLUE_CUBES: usize = 14;
/// Maximum amount of Green Cubes that can be in any samples
const GREEN_CUBES: usize = 13;
/// Structure representing a "Game" as described in Day 2
#[derive(Debug)]
pub struct Game {
    pub id: String,
    // shown samples
    pub samples: Vec<String>,
}
impl Game {
    /// Creates a new "Game" as described in Day 2. Will panic if the String is not in the expected format of
    /// "Game 2: 1 red, 2 green, 3 blue" ...
    pub fn new(s: &str) -> Self {
        // We take the given string an split it at ":"
        // this should leave us with the Game ID as a String and another string with all the samples given
        // by the elf
        let mut id = s.split(':');
        let game_string = if let Some(s) = id.next() {
            s
        } else {
            panic!("Splitting with : must yield exactly two sub strings");
        };
        let samples_strings = if let Some(s) = id.next() {
            s.split(';')
        } else {
            panic!("Splitting with : must yield exactly two sub strings");
        };
        assert!(id.next().is_none(), "Unexpected amounts if sub strings");
        // In the next step we will put all samples separated in a Vec of Strings
        let mut samples = Vec::new();
        for sample in samples_strings {
            samples.push(String::from(sample));
        }
        // Create the "Game"
        // all further operations will work on this pre processed form
        Game {
            id: String::from(game_string),
            samples,
        }
    }
    /// Calculates the results as described in the first part of Day 2
    ///
    /// Checks the given Cube Configuration against the given String. It checks that any samples does not has more cubes in it than
    /// possible by the configuration
    pub fn get_result(&self) -> bool {
        for sample in &self.samples {
            let res = get_sample_result(sample);
            if res.red > RED_CUBES || res.blue > BLUE_CUBES || res.green > GREEN_CUBES {
                return false;
            }
        }
        true
    }

    /// Calculates the smallest game product as it is described in Part 2 of Day 2
    ///
    /// Checks the least amount of cube configuration that would be possible for the given samples
    /// it multiplies all three smallest possible configuration and returns that
    pub fn get_smallest_game_sum(&self) -> usize {
        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;
        for sample in &self.samples {
            let res = get_sample_result(sample);
            if res.red > min_red {
                min_red = res.red;
            }
            if res.green > min_green {
                min_green = res.green;
            }
            if res.blue > min_blue {
                min_blue = res.blue;
            }
        }
        min_blue * min_red * min_green
    }

    pub fn get_id(&self) -> usize {
        extract_id(&self.id)
    }
}

/// Takes the Game ID String from [Game] and extracts the ID as usize
///
/// Removes all non ASCII Digits chars and parses the left numbers
fn extract_id(s: &str) -> usize {
    let mut s = String::from(s);
    s.retain(|c| c.is_ascii_digit());
    s.parse::<usize>().unwrap()
}
/// Analyses a single given sample and returns the number of present cubes
///
/// By splitting with "," all yielding substrings describe one cube color
/// We iterate over all substrings and check which color it maches
/// This could for sure be done more optimally as we re check every color even
/// if we already found it once in sample.
fn get_sample_result(s: &str) -> GameResult {
    let single_results = s.split(',');
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for result in single_results {
        let red_index = result.find("red");
        if let Some(index) = red_index {
            let mut s = String::from(result.split_at(index).0);
            s.retain(|c| !c.is_whitespace());
            red = s.parse::<usize>().unwrap();
        }

        let blue_index = result.find("blue");
        if let Some(index) = blue_index {
            let mut s = String::from(result.split_at(index).0);
            s.retain(|c| !c.is_whitespace());
            blue = s.parse::<usize>().unwrap();
        }
        let green_index = result.find("green");
        if let Some(index) = green_index {
            let mut s = String::from(result.split_at(index).0);
            s.retain(|c| !c.is_whitespace());
            green = s.parse::<usize>().unwrap();
        }
    }
    GameResult { red, blue, green }
}
#[derive(Debug, PartialEq)]
struct GameResult {
    pub red: usize,
    pub blue: usize,
    pub green: usize,
}
impl Add for GameResult {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            blue: self.blue + rhs.blue,
            green: self.green + rhs.green,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn new_game() {
        let game_str = "Game 2: 4 red, 5 green, 2 blue; 7 red, 14 green, 3 blue; 2 green, 5 blue, 11 red; 10 blue, 3 green; 9 green, 6 blue, 13 red; 7 red, 5 green, 9 blue";
        let game = Game::new(game_str);

        assert_eq!(game.id, "Game 2");
        assert_eq!(game.samples.get(0).unwrap(), " 4 red, 5 green, 2 blue");
    }
    #[test]
    fn test_get_sample_result() {
        let s = " 4 red, 5 green, 2 blue";
        assert_eq!(
            GameResult {
                red: 4,
                blue: 2,
                green: 5
            },
            get_sample_result(s)
        );

        let s = " 7 red, 1 green, 4 blue";
        assert_eq!(
            GameResult {
                red: 7,
                blue: 4,
                green: 1
            },
            get_sample_result(s)
        );

        let s = " 1 green, 4 blue";
        assert_eq!(
            GameResult {
                red: 0,
                blue: 4,
                green: 1
            },
            get_sample_result(s)
        );
        let s = " 1 green, 4 blue, 7 red";
        assert_eq!(
            GameResult {
                red: 7,
                blue: 4,
                green: 1
            },
            get_sample_result(s)
        );
    }

    #[test]
    fn test_get_result() {
        let s = "Game2: 4 red, 5 green, 2 blue; 7 red, 14 green, 3 blue; 2 green, 5 blue, 11 red; 10 blue, 3 green; 9 green, 6 blue, 13 red; 7 red, 5 green, 9 blue";
        let res = Game::new(s);
        let res = res.get_result();
        assert_eq!(res, false);
    }

    #[test]
    fn test_get_id() {
        let s = "Game 2:";
        assert_eq!(2, extract_id(s));

        let s = "Game 20:";
        assert_eq!(20, extract_id(s));

        let s = "Game 205:";
        assert_eq!(205, extract_id(s));

        let s = "Game 2050:";
        assert_eq!(2050, extract_id(s));
    }
}
