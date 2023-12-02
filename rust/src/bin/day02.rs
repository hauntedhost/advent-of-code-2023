use advent_of_code_2023::*;
use std::collections::HashMap;

// Day 2: Cube Conundrum
// https://adventofcode.com/2023/day/2

fn main() {
    let lines = read_lines_from_file("02-input.txt");

    // Part One
    // ========
    let limits: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut games: Vec<i32> = Vec::new();

    // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    'lines: for line in &lines {
        let parts: Vec<&str> = line.split(":").collect();
        let game_id = parts[0][5..].parse::<i32>().unwrap();

        // split rounds on ";"
        // e.g., ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
        let rounds: Vec<&str> = parts[1]
            .split(";")
            .map(|s| s.trim())
            .collect();

        for round in rounds {
            // split pairs on ","
            // e.g. ["1", "red"]
            for pair in round.split(",") {
                let parts: Vec<&str> = pair.trim().split_whitespace().collect();
                let num = parts[0].parse::<i32>().unwrap();
                let color = parts[1];

                // skip this line if num is greater than limit for that color
                if &num > limits.get(color).unwrap() {
                    continue 'lines;
                }
            }
        }

        // valid game
        games.push(game_id);
    }

    let sum: i32 = games.iter().sum();
    println!("{:?}", sum);

    // Part Two
    // ========
    let mut games: Vec<i32> = Vec::new();

    // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    for line in &lines {
        let parts: Vec<&str> = line.split(":").collect();

        // split rounds on ";"
        // e.g. ["3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
        let rounds: Vec<&str> = parts[1]
            .split(";")
            .map(|s| s.trim())
            .collect();

        let mut minimums: HashMap<&str, i32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        for round in rounds {
            // split pairs on ","
            // e.g. ["1", "red"]
            for pair in round.split(",") {
                let parts: Vec<&str> = pair.trim().split_whitespace().collect();
                let num = parts[0].parse::<i32>().unwrap();
                let color = parts[1];

                // update minimum for current color if num is greater
                if &num > minimums.get(color).unwrap() {
                    minimums.insert(color, num);
                }
            }
        }

        // push the cube power
        let power = minimums.values().fold(1, |acc, &val| acc * val);
        games.push(power);
    }

    let sum: i32 = games.iter().sum();
    println!("{:?}", sum);
}
