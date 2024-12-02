use std::fs::read_to_string;

use itertools::Itertools;

fn main() {
    let out = read_to_string("aoc_12_02.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|item| item.parse::<i32>().unwrap())
                .collect_vec()
        })
        .map(|item| report_safe_with_dapener(&item) as u32)
        .sum::<u32>();

    dbg!(out);
}

fn report_safe(report: &[i32]) -> bool {
    let mut iter = report.windows(2).map(|items| items[0] - items[1]);
    let first = iter.next().unwrap();
    if first == 0 {
        return false;
    }
    if first.abs() > 3 {
        return false;
    }
    iter.all(|diff| {
        if first.signum() != diff.signum() {
            false
        } else if diff.abs() > 3 {
            false
        } else {
            true
        }
    })
}

fn report_safe_with_dapener(report: &[i32]) -> bool {
    let mut iter = report.windows(2).map(|items| items[0] - items[1]);
    let first = iter.next().unwrap();
    let first_safe = if first == 0 {
        false
    } else if first.abs() > 3 {
        false
    } else {
        true
    };

    if !first_safe {
        let mut first_dapaned_report = report.to_vec();
        first_dapaned_report.remove(0);
        let first_report_safe = report_safe(&first_dapaned_report);
        if first_report_safe {
            return true;
        }
        let mut second_dapaned_report = report.to_vec();
        second_dapaned_report.remove(1);
        return report_safe(&second_dapaned_report);
    }
    let problem_indices = iter
        .map(|diff| {
            if first.signum() != diff.signum() {
                false
            } else if diff.abs() > 3 {
                false
            } else {
                true
            }
        })
        .enumerate()
        .filter_map(|(index, safe)| if safe { None } else { Some(index + 1) })
        .collect_vec();
    if problem_indices.is_empty() {
        return true;
    }
    for problem_index in problem_indices {
        let mut first_dapaned_report = report.to_vec();
        first_dapaned_report.remove(problem_index);
        if report_safe(&first_dapaned_report) {
            return true;
        }
        let mut second_dapaned_report = report.to_vec();
        second_dapaned_report.remove(problem_index + 1);
        if report_safe(&second_dapaned_report) {
            return true;
        }
    }

    let mut first_dapaned_report = report.to_vec();
    first_dapaned_report.remove(0);
    let first_report_safe = report_safe(&first_dapaned_report);
    if first_report_safe {
        return true;
    }
    let mut second_dapaned_report = report.to_vec();
    second_dapaned_report.remove(1);
    return report_safe(&second_dapaned_report);
}
