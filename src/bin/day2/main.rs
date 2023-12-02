mod game;

use game::Game;

fn main() {
    let file = std::fs::read_to_string("./src/bin/day2/input.txt").unwrap();
    let lines = file.lines();
    let mut sum = 0;
    for line in lines {
        // Create a Game from every Line
        let game = Game::new(line);
        let res = game.get_result();
        if res == true {
            sum += game.get_id();
        }
    }
    println!("First sum {}", sum);
    sum = 0;
    let file = std::fs::read_to_string("./src/bin/day2/input.txt").unwrap();
    let lines = file.lines();
    for line in lines {
        let game = Game::new(line);
        let res = game.get_smallest_game_sum();
        sum += res;
    }
    println!("Second sum {}", sum);
}

#[test]
fn reference() {
    let file = std::fs::read_to_string("./src/bin/day2/test.txt").unwrap();
    let lines = file.lines();
    let mut sum = 0;
    for line in lines {
        // Create a Game from every Line
        let game = Game::new(line);
        let res = game.get_result();
        if res == true {
            sum += game.get_id();
            eprintln!("Id {} valid game with {:?}", game.get_id(), res);
        }
    }
    eprintln!("sum {}", sum);
    assert_eq!(sum, 8);
}

#[test]
fn reference2() {
    let file = std::fs::read_to_string("./src/bin/day2/test.txt").unwrap();
    let lines = file.lines();
    let mut sum = 0;
    for line in lines {
        // Create a Game from every Linje
        let game = Game::new(line);
        let res = game.get_smallest_game_sum();
        sum += res;
    }
    eprintln!("sum {}", sum);
    assert_eq!(sum, 2286);
}
