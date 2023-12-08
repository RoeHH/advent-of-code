use std::{borrow::BorrowMut, fs};

pub fn day(challenge: u8) {
    let contents =
        fs::read_to_string("../data/1.txt").expect("Should have been able to read the file");
    let contents = contents
        .split("\n")
        .map(|line| {
            if challenge == 2 {
                return line
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "4")
                    .replace("five", "5e")
                    .replace("six", "6")
                    .replace("seven", "7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e");
            } else {
                return line.to_owned();
            }
        })
        .map(|line| line.chars().map(|c| c.to_string()).collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>();

    let mut sum = 0;
    for line in contents {
        sum += (find_first(&line) + find_last(&line).borrow_mut())
            .parse::<i32>()
            .unwrap();
    }
    println!("Day 1, challenge {}: {}", challenge, sum);
}

fn find_first(line: &Vec<String>) -> String {
    let mut iter = 0;
    loop {
        if line[iter].chars().all(char::is_numeric) {
            return line[iter].to_owned();
        }
        iter += 1;
    }
}

fn find_last(line: &Vec<String>) -> String {
    let mut iter = line.len() - 1;
    loop {
        if line[iter].chars().all(char::is_numeric) {
            return line[iter].to_owned();
        }
        iter -= 1;
    }
}
