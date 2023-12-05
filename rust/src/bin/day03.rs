use advent_of_code_2023::*;
use std::collections::HashMap;

// Day 3: Gear Ratios
// https://adventofcode.com/2023/day/3

fn get_in(
    matrix: &HashMap<usize, HashMap<usize, char>>,
    row: usize,
    col: usize
) -> Option<char> {
    matrix.get(&row).and_then(|row_map| row_map.get(&col).cloned())
}

fn is_symbol_at(
    matrix: &HashMap<usize, HashMap<usize, char>>,
    row: usize,
    col: usize
) -> bool {
    if let Some(char) = get_in(&matrix, row, col) { is_symbol(char) } else { false }
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.' && !c.is_whitespace()
}

fn sum_chars(chars: &[char]) -> i32 {
    chars.iter().collect::<String>().parse::<i32>().unwrap()
}

fn is_adjacent_to_symbol(
    matrix: &HashMap<usize, HashMap<usize, char>>,
    row: usize,
    start_col: usize,
    end_col: usize
) -> bool {
    const OFFSETS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for &(row_offset, col_offset) in &OFFSETS {
        let start_col = (start_col as i32) + col_offset;
        let end_col = (end_col as i32) + col_offset - 1;

        for adj_col in start_col..=end_col {
            let adj_row = (row as i32) + row_offset;
            if adj_row >= 0 && adj_col >= 0 {
                if is_symbol_at(matrix, adj_row as usize, adj_col as usize) {
                    return true;
                }
            }
        }
    }

    false
}

fn main() {
    let lines = read_lines_from_file("03-input.txt");

    // Part One
    // ========

    // build a nested hash lookup of chars
    // not sure if this is the best data structure in rust for this 🤷🏻‍♀️
    let mut matrix: HashMap<usize, HashMap<usize, char>> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let mut row_map: HashMap<usize, char> = HashMap::new();
        for (j, c) in line.chars().enumerate() {
            row_map.insert(j, c);
        }
        matrix.insert(i, row_map);
    }

    // collect part numbers
    // i could just build the sum as i go, but i wanna be able to see the part numbers 🤓
    let mut part_numbers = vec![];

    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;
        let row: Vec<char> = line.chars().collect();

        while j < row.len() {
            if row[j].is_numeric() {
                let mut k = j + 1;

                while k < row.len() && row[k].is_numeric() {
                    k += 1;
                }

                if is_adjacent_to_symbol(&matrix, i, j, k) {
                    let number = sum_chars(&row[j..k]);
                    part_numbers.push(number);
                }

                j = k;
            } else {
                j = j + 1;
            }
        }
    }

    let sum: i32 = part_numbers.iter().sum();
    println!("Sum of part numbers: {}", sum);
}
