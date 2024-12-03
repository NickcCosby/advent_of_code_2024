use std::fs::read_to_string;

use regex::{Match, Regex};

fn main() {
    puzzle_2();
}

fn puzzle_1() {
    let input = read_to_string("aoc_12_03.txt").unwrap();

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let out: i32 = re
        .captures_iter(&input)
        .map(|capture| {
            let parse = |group: Option<Match>| group.unwrap().as_str().parse::<i32>().unwrap();
            parse(capture.get(1)) * parse(capture.get(2))
        })
        .sum();

    dbg!(out);
}

fn puzzle_2() {
    let input = read_to_string("aoc_12_03.txt").unwrap();

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();

    // let mut acc: i32 = 0;
    // let mut mul_on = true;

    // for capture in re.captures_iter(&input) {
    //     if capture.get(0).unwrap().as_str() == "do()" {
    //         mul_on = true;
    //     } else if capture.get(0).unwrap().as_str() == "don't()" {
    //         mul_on = false;
    //     } else if mul_on {
    //         let parse = |group: Option<Match>| group.unwrap().as_str().parse::<i32>().unwrap();
    //         let result = parse(capture.get(1)) * parse(capture.get(2));
    //         acc += result;
    //     }
    // }

    let (_, out) = re
        .captures_iter(&input)
        .fold((true, 0), |(mul_on, acc), capture| {
            if capture.get(0).unwrap().as_str() == "do()" {
                (true, acc)
            } else if capture.get(0).unwrap().as_str() == "don't()" {
                (false, acc)
            } else if mul_on {
                let parse = |group: Option<Match>| group.unwrap().as_str().parse::<i32>().unwrap();
                let result = parse(capture.get(1)) * parse(capture.get(2));
                (mul_on, acc + result)
            } else {
                (mul_on, acc)
            }
        });

    dbg!(out);
}
