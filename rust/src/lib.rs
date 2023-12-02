use std::fs;

pub fn read_lines_from_file(filename: &str) -> Vec<String> {
    let filename = format!("{}{}", "../data/", filename);

    fs::read_to_string(filename)
        .expect("Failed to read input")
        .trim_end_matches(&['\r', '\n'][..])
        .split("\n")
        .map(|line| line.to_string())
        .collect()
}
