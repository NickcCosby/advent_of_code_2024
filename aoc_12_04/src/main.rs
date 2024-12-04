use std::fs::read_to_string;

use ndarray::{Array2, ArrayView2};

fn main() {
    let input = read_to_string("aoc_12_04.txt").unwrap();

    let width = input.lines().next().unwrap().len();
    let height = input.len() / width;

    //Can this be gotten rid of?
    let input: Vec<_> = input.chars().collect();

    let input = Array2::from_shape_vec((width, height), input).unwrap();
    puzzle_2(input);
}

fn puzzle_1(input: Array2<char>) {
    let mut acc = 0;

    for ((col, row), char) in input.indexed_iter() {
        if *char == 'X' {
            for direction in check_adjacent(col, row, input.view(), 'M') {
                if let Some(m_offset) = direction.offset(col, row) {
                    if let Some(a_offset) = direction.offset(m_offset.0, m_offset.1) {
                        if let Some(s_offset) = direction.offset(a_offset.0, a_offset.1) {
                            if input.get(a_offset) == Some(&'A')
                                && input.get(s_offset) == Some(&'S')
                            {
                                acc += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    dbg!(acc);
}

fn puzzle_2(input: Array2<char>) {
    let out =
        input
            .indexed_iter()
            .filter(|(_, &char)| char == 'A')
            .fold(0, |acc, ((col, row), char)| {
                let adjacents = check_adjacent(col, row, input.view(), 'M');
                let adjacents: Vec<_> = adjacents
                    .into_iter()
                    .filter(|direction| {
                        matches!(
                            direction,
                            AdjacentDirection::NorthEast
                                | AdjacentDirection::NorthWest
                                | AdjacentDirection::SouthEast
                                | AdjacentDirection::SouthWest
                        )
                    })
                    .collect();

                if adjacents.len() == 2 {
                    if let Some(s_1_offset) = adjacents[0].reverse().offset(col, row) {
                        if let Some(s_2_offset) = adjacents[1].reverse().offset(col, row) {
                            if input.get(s_1_offset) == Some(&'S')
                                && input.get(s_2_offset) == Some(&'S')
                            {
                                return acc + 1;
                            }
                        }
                    }
                }
                return acc;
            });

    dbg!(out);
}

enum AdjacentDirection {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl AdjacentDirection {
    fn offset(&self, col: usize, row: usize) -> Option<(usize, usize)> {
        let (col, row) = (col as i32, row as i32);
        let (col, row) = match self {
            AdjacentDirection::North => (col, row - 1),
            AdjacentDirection::NorthWest => (col + 1, row - 1),
            AdjacentDirection::West => (col + 1, row),
            AdjacentDirection::SouthWest => (col + 1, row + 1),
            AdjacentDirection::South => (col, row + 1),
            AdjacentDirection::SouthEast => (col - 1, row + 1),
            AdjacentDirection::East => (col - 1, row),
            AdjacentDirection::NorthEast => (col - 1, row - 1),
        };
        if col.is_negative() || row.is_negative() {
            None
        } else {
            Some((col as usize, row as usize))
        }
    }

    fn reverse(&self) -> Self {
        match self {
            AdjacentDirection::North => AdjacentDirection::South,
            AdjacentDirection::NorthWest => AdjacentDirection::SouthEast,
            AdjacentDirection::West => AdjacentDirection::East,
            AdjacentDirection::SouthWest => AdjacentDirection::NorthEast,
            AdjacentDirection::South => AdjacentDirection::North,
            AdjacentDirection::SouthEast => AdjacentDirection::NorthWest,
            AdjacentDirection::East => AdjacentDirection::West,
            AdjacentDirection::NorthEast => AdjacentDirection::SouthWest,
        }
    }

    fn check_set() -> impl Iterator<Item = Self> {
        let mut count = 0;
        std::iter::from_fn(move || {
            let curr_count = count;
            count += 1;
            match curr_count {
                0 => Some(AdjacentDirection::North),
                1 => Some(AdjacentDirection::NorthWest),
                2 => Some(AdjacentDirection::West),
                3 => Some(AdjacentDirection::SouthWest),
                4 => Some(AdjacentDirection::South),
                5 => Some(AdjacentDirection::SouthEast),
                6 => Some(AdjacentDirection::East),
                7 => Some(AdjacentDirection::NorthEast),
                _ => None,
            }
        })
    }
}

fn check_adjacent(
    col: usize,
    row: usize,
    input: ArrayView2<char>,
    target: char,
) -> Vec<AdjacentDirection> {
    AdjacentDirection::check_set()
        .filter_map(|direction| {
            if input.get(direction.offset(col, row)?) == Some(&target) {
                Some(direction)
            } else {
                None
            }
        })
        .collect()
}
