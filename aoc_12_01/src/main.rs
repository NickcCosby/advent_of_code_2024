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
            let mut out = line.split("   ");
            (
                out.next().unwrap().to_owned().parse::<i32>().unwrap(),
                out.next().unwrap().to_owned().parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

fn puzzle_1() {
    let (mut first_list, mut second_list) = parse_lists("aoc_12_01_1_input.txt");
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
    let (first_list, second_list) = parse_lists("aoc_12_01_1_input.txt");
    let mut first_list: HashMap<i32, i32> = first_list.into_iter().map(|item| (item, 0)).collect();
    for item in second_list {
        first_list.entry(item).and_modify(|acc| *acc += item);
    }
    let sum = first_list
        .into_iter()
        .map(|(key, value)| value)
        .sum::<i32>();

    dbg!(sum);
}