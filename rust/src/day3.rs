use std::{fs, collections::HashSet, vec};

pub fn day(challenge: u8) {
    let contents =
        &fs::read_to_string("../data/3.txt").expect("Should have been able to read the file");
    let borderd_content = ".".repeat(142).to_owned()
        + "\n."
        + &contents.replace("\n", ".\n.")
        + "\n"
        + &".".repeat(142);

    let lines: Vec<Vec<&str>> = borderd_content
        .split("\n")
        .map(|line| {
            line.split("")
                .filter(|c| c != &"" && c != &"\r")
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    if challenge == 1 {
        println!("Day 3, challenge {}: {}", challenge, p1(lines));
    } else {
        println!("Day 3, challenge {}: {}", challenge, p2(lines));
    }
}

fn p1(lines: Vec<Vec<&str>>) -> i32 {
    let mut number: String = "".to_string();
    let mut sum = 0;
    for y in 1..(lines.len() - 1) {
        let mut adjacent_symbol = false;
        for x in 1..(lines[y].len() - 1) {
            if lines[y][x].chars().all(char::is_numeric) {
                let adjacent_symbols_count = vec![
                    lines[y][x - 1],
                    lines[y][x + 1],
                    lines[y - 1][x],
                    lines[y + 1][x],
                    lines[y - 1][x - 1],
                    lines[y - 1][x + 1],
                    lines[y + 1][x - 1],
                    lines[y + 1][x + 1],
                ]
                .iter()
                .filter(|c| !c.chars().all(char::is_numeric) && !c.eq(&&"."))
                .count();
                if adjacent_symbols_count > 0 {
                    adjacent_symbol = true;
                }
                number = number.to_string() + lines[y][x]
            } else if adjacent_symbol {
                sum += number.parse::<i32>().unwrap();
                number = "".to_string();
                adjacent_symbol = false;
            } else {
                number = "".to_string();
            }
        }
        if adjacent_symbol {
            sum += number.parse::<i32>().unwrap();
            number = "".to_string();
        } else {
            number = "".to_string();
        }
    }
    return sum;
}

fn p2(lines: Vec<Vec<&str>>) -> usize {
    let mut sum = 0;
    for y in 1..(lines.len() - 1) {
        for x in 1..(lines[y].len() - 1) {
            if lines[y][x] == "*" {
                let adjacent_numbers = get_adjacent_numbers(&lines, x, y);
                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers[0] * adjacent_numbers[1];
                }
            } 
        }
    }
    return sum;
}

fn get_adjacent_numbers(lines: &Vec<Vec<&str>>, x: usize, y: usize) -> Vec<usize> {
    let mut possible_adjacent_numbers = HashSet::<(usize, usize)>::new();
    let _: Vec<_> = vec![(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
        .iter()
        .map(|(map_x, map_y)| get_start_pos_from_pos(lines, map_x, map_y).inspect(|pos| {
            possible_adjacent_numbers.insert(*pos);        
        }))
        .collect();
    return possible_adjacent_numbers.into_iter()
        .map(|(x, y)| {
            let mut num = String::from(lines[y][x]);
            let mut x = x + 1;
            while lines[y][x].chars().all(char::is_numeric) {
                num.push_str(lines[y][x]);
                x += 1;
            }
            return num.parse::<usize>().expect("Should be a Number");
        })
    .collect::<Vec<usize>>()
}

fn get_start_pos_from_pos(
    lines: &Vec<Vec<&str>>,
    x: &usize,
    y: &usize
) -> Option<(usize, usize)> {
    let mut x = x.to_owned();
    if !lines[*y][x].chars().all(char::is_numeric) {
        return None;
    }
    while lines[*y][x-1].chars().all(char::is_numeric) {
        x -= 1;        
    }
    return Some((x, *y));
}
