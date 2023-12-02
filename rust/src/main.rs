use std::collections::HashMap;
use std::fs;

fn read_lines_from_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .expect("Failed to read input")
        .trim_end_matches(&['\r', '\n'][..])
        .split("\n")
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    // Part One
    // ========
    let lines = read_lines_from_file("../data/01-input.txt");

    let mut calibrations: Vec<i32> = Vec::new();

    lines.iter().for_each(|line| {
        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for c in line.chars() {
            if c.is_numeric() {
                if first.is_none() {
                    first = Some(c);
                } else {
                    last = Some(c);
                }
            }
        }

        if last.is_none() {
            last = first;
        }

        // horrifying
        let calibration = (first.unwrap().to_string() + &last.unwrap().to_string())
            .parse::<i32>()
            .unwrap();

        calibrations.push(calibration);
    });

    let sum: i32 = calibrations.iter().sum();
    // println!("{:?}", calibrations);
    println!("{:?}", sum);

    // Part Two
    // ========
    let lines = read_lines_from_file("../data/01-input.txt");

    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut calibrations: Vec<i32> = Vec::new();

    for line in &lines {
        // build a map of matching indexes to numbers, e.g.
        // "two1nine" matches => {
        //   0: 2,
        //   3: 1,
        //   4: 9
        // }
        let mut matches: HashMap<usize, usize> = HashMap::new();

        for (index, char) in line.char_indices() {
            // if the character is numeric, put that as match and continue
            if char.is_numeric() {
                let num = char.to_digit(10).unwrap() as usize;
                matches.insert(index, num);
                continue;
            } else {
                // else loop through nums and see if any word matches
                for (index, word) in nums.iter().enumerate() {
                    let num = index + 1;
                    if line[index..].starts_with(word) {
                        matches.insert(index, num);
                        break;
                    }
                }
            }
        }

        // get values in matches map for min/max keys
        // e.g. "two1nine" matches => {0: 2, 3: 1, 4: 9}
        // min key is 0 with a 2 value, max key is 4, with a 9 value
        let first = matches
            .keys()
            .min()
            .and_then(|&min_key| matches.get(&min_key))
            .unwrap();

        let last = matches
            .keys()
            .max()
            .and_then(|&max_key| matches.get(&max_key))
            .unwrap();

        // concat first and last as strings and convert to i32 for calibration
        let calibration = format!("{}{}", first, last).parse::<i32>().unwrap();

        calibrations.push(calibration);
    }

    let sum: i32 = calibrations.iter().sum();
    println!("{:?}", sum);
}
