use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    // let lines = lines_from_file("input.txt");
    let count = count_of_greater_than_prev(lines_from_file("input.txt"));
    println!["part1 {}", count];
    let count = group_count_of_greater_than_prev(lines_from_file("input.txt"));
    println!["part2 {}", count];
}

fn count_of_greater_than_prev(values: Vec<i32>) -> i32 {
    let mut current = 2147483647;
    let mut count = 0;
    for value in values {
        if value > current {
            count += 1;
        }
        current = value;
    }
    return count;
}

fn create_groups(values: Vec<i32>) -> Vec<i32> {
    let mut count1 = 0;
    let mut count2 = -1;
    let mut count3 = -2;
    let mut windowedSum1 = 0;
    let mut windowedSum2 = 0;
    let mut windowedSum3 = 0;
    let mut windowed = Vec::new();
    for value in values {
        if count1 >= 0 {
            windowedSum1 += value;
        }
        if count2 >= 0 {
            windowedSum2 += value;
        }
        if count3 >= 0 {
            windowedSum3 += value;
        }
        count1 += 1;
        count2 += 1;
        count3 += 1;
        if count1 == 3 {
            windowed.push(windowedSum1);
            windowedSum1 = 0;
            count1 = 0;
        }
        if count2 == 3 {
            windowed.push(windowedSum2);
            windowedSum2 = 0;
            count2 = 0;
        }
        if count3 == 3 {
            windowed.push(windowedSum3);
            windowedSum3 = 0;
            count3 = 0;
        }
    }
    return windowed;
}

fn group_count_of_greater_than_prev(values: Vec<i32>) -> i32 {
    let groupedValues = create_groups(values);
    return count_of_greater_than_prev(groupedValues);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

#[test]
fn part1_test() {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let answer = count_of_greater_than_prev(v);

    assert_eq!(7, answer);
}

#[test]
fn part2_windowedGeneratorTest() {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let answer = create_groups(v);
    let expected = vec![607, 618, 618, 617, 647, 716, 769, 792];
    assert_eq!(expected, answer);
}

#[test]
fn part2_test() {
    let v = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    let answer = group_count_of_greater_than_prev(v);

    assert_eq!(5, answer);
}