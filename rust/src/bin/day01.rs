use advent_of_code_2023::*;
use std::collections::HashMap;

// Day 1: Trebuchet?!
// https://adventofcode.com/2023/day/1

fn main() {
    // Part One
    // ========
    let lines = read_lines_from_file("01-input.txt");

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
    println!("{:?}", sum);

    // Part Two
    // ========
    let lines = read_lines_from_file("01-input.txt");

    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut calibrations: Vec<i32> = Vec::new();

    for line in &lines {
        // build a map of matching indexes to numbers
        // e.g. "two1nine" would be:
        //   {
        //     0: 2,
        //     3: 1,
        //     4: 9
        //   }

        let mut matches: HashMap<usize, usize> = HashMap::new();
        let mut skip_until = None;

        for (i, char) in line.char_indices() {
            // optimization skip
            if let Some(skip_index) = skip_until {
                if i < skip_index {
                    continue;
                }
                skip_until = None;
            }

            // if the character is numeric, put that as match and continue
            if char.is_numeric() {
                let num = char.to_digit(10).unwrap() as usize;
                matches.insert(i, num);
                continue;
            } else {
                // else loop through nums and see if any word matches
                for (j, word) in nums.iter().enumerate() {
                    let num = j + 1;
                    if line[i..].starts_with(word) {
                        matches.insert(i, num);
                        skip_until = Some(i + word.len() - 1);
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
