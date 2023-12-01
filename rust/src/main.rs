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
    let lines = read_lines_from_file("../data/01-input1.txt");

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
    let lines = read_lines_from_file("../data/01-input2.txt");

    // let's make a map of words to numbers
    let number_words = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut calibrations: Vec<i32> = Vec::new();

    for line in &lines {
        // we're building a map of matching indices to numbers, e.g.
        // "two1nine" => {0: "2", 3: "1", 4: "9"}
        let mut match_map: HashMap<usize, String> = HashMap::new();

        for (i, c) in line.char_indices() {
            // if the character is numeric, put that as match and continue
            if c.is_numeric() {
                match_map.insert(i, c.to_string());
                continue;
            } else {
                // else loop through lookup map and see if any word matches
                for (word, &number) in &number_words {
                    if line[i..].starts_with(word) {
                        match_map.insert(i, number.to_string());
                        break;
                    }
                }
            }
        }

        // now with this map, we take min/max keys and get the values
        // e.g. "two1nine" match_map => {0: "2", 3: "1", 4: "9"}
        // min key => 0 with "2" value, max key => 4, with "9" value

        // why are you so rust :'(
        let first = match_map
            .keys()
            .min()
            .and_then(|&min_key| match_map.get(&min_key))
            .unwrap()
            .to_string();

        let last = match_map
            .keys()
            .max()
            .and_then(|&max_key| match_map.get(&max_key))
            .unwrap()
            .to_string();

        let calibration = (first + &last).parse::<i32>().unwrap();

        calibrations.push(calibration);
    }

    let sum: i32 = calibrations.iter().sum();
    // println!("{:?}", calibrations);
    println!("{:?}", sum);
}
