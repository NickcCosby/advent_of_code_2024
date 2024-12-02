use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{BufReader, Read},
};

fn main() {
    puzzle_2();
}

fn parse_lists(file_name: &str) -> (Vec<i32>, Vec<i32>) {
    read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let out = line.split_once("   ").unwrap();
            (out.0.parse::<i32>().unwrap(), out.1.parse::<i32>().unwrap())
        })
        .unzip()
}

fn puzzle_1() {
    let (mut first_list, mut second_list) = parse_lists("aoc_12_01_input.txt");
    first_list.sort();
    second_list.sort();
    let sum = first_list
        .into_iter()
        .zip(second_list)
        .map(|(first, second)| (first - second).abs())
        .sum::<i32>();
    println!("{}", sum);
}

fn puzzle_2() {
    let (first_list, second_list) = parse_lists("aoc_12_01_input.txt");
    let mut first_list: HashMap<i32, i32> = first_list.into_iter().map(|item| (item, 0)).collect();
    for item in second_list {
        first_list.entry(item).and_modify(|acc| *acc += item);
    }
    let sum: i32 = first_list.values().sum();

    dbg!(sum);
}
