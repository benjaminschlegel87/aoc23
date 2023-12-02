use std::ops::Add;

const RED_CUBES: usize = 12;
const BLUE_CUBES: usize = 14;
const GREEN_CUBES: usize = 13;
#[derive(Debug)]
pub struct Game {
    pub id: String,
    // shown samples
    pub samples: Vec<String>,
}
impl Game {
    pub fn new(s: &str) -> Self {
        let mut id = s.split(":");
        let game_string = id.next().unwrap();
        let samples_strings = id.next().unwrap().split(";");
        assert!(id.next().is_none());
        let mut samples = Vec::new();
        for sample in samples_strings {
            samples.push(String::from(sample));
        }
        Game {
            id: String::from(game_string),
            samples,
        }
    }
    pub fn get_result(&self) -> bool {
        for sample in &self.samples {
            let res = get_sample_result(sample);
            if res.red > RED_CUBES || res.blue > BLUE_CUBES || res.green > GREEN_CUBES {
                return false;
            }
        }
        true
    }

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

fn extract_id(s: &str) -> usize {
    let mut s = String::from(s);
    s.retain(|c| c.is_ascii_digit());
    s.parse::<usize>().unwrap()
}
// " 4 red, 5 green, 2 blue"
fn get_sample_result(s: &str) -> GameResult {
    let single_results = s.split(",");
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
pub struct GameResult {
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
