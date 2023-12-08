use std::fs;

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
                } else {
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

fn p2(lines: Vec<Vec<&str>>) -> i32 {
    let mut sum = 0;
    for y in 1..(lines.len() - 1) {
        for x in 1..(lines[y].len() - 1) {
            if lines[y][x] == "*" {
                let adjacent_numbers = get_adjacent_numbers(&lines, x, y);
                if adjacent_numbers.len() == 2 {
                    sum += adjacent_numbers[0] * adjacent_numbers[1];
                    println!(
                        "{}*{}={} ",
                        adjacent_numbers[0],
                        adjacent_numbers[1],
                        adjacent_numbers[0] * adjacent_numbers[1]
                    )
                }
            }
        }
    }
    return sum;
}

fn get_adjacent_numbers(lines: &Vec<Vec<&str>>, x: usize, y: usize) -> Vec<i32> {
    let possible_adjacent_numbers = vec![
        get_three_digit_num_from_pos(lines, x - 1, y - 1, false),
        get_three_digit_num_from_pos(lines, x - 2, y - 1, false),
        get_three_digit_num_from_pos(lines, x - 3, y - 1, false),
        get_three_digit_num_from_pos(lines, x, y - 1, false),
        get_three_digit_num_from_pos(lines, x + 1, y - 1, true),
        get_three_digit_num_from_pos(lines, x - 3, y, false),
        get_three_digit_num_from_pos(lines, x, y, false),
        get_three_digit_num_from_pos(lines, x + 1, y, true),
        get_three_digit_num_from_pos(lines, x - 1, y + 1, false),
        get_three_digit_num_from_pos(lines, x - 2, y + 1, false),
        get_three_digit_num_from_pos(lines, x - 3, y + 1, false),
        get_three_digit_num_from_pos(lines, x, y + 1, false),
        get_three_digit_num_from_pos(lines, x + 1, y + 1, true),
    ];
    return possible_adjacent_numbers
        .iter()
        .filter(|n| {
            n.chars().all(char::is_numeric) && // Check if it's a numeric string
            !possible_adjacent_numbers.iter().any(|m| {
                m != n.to_owned() && m.contains(n.to_owned())
            })
        })
        .map(|n| n.parse::<i32>().expect("Should be a Number"))
        .collect::<Vec<i32>>();
}

fn get_three_digit_num_from_pos(
    lines: &Vec<Vec<&str>>,
    x: usize,
    y: usize,
    left_side: bool,
) -> String {
    if x < 1 || y < 1 || x > lines[y].len() - 1 || y > lines.len() - 1 {
        return "Out of Bounds".to_string();
    }
    let mut possible_number: String = "".to_string();
    for i in 0..3 {
        possible_number = possible_number.to_string() + lines[y][x + i]
    }
    if possible_number.chars().all(char::is_numeric) {
        return possible_number;
    }
    let mut possible_number: String = "".to_string();
    if left_side {
        for i in 1..3 {
            possible_number = possible_number.to_string() + lines[y][x + i]
        }
    }
    return possible_number;
}
