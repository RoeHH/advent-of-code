use std::fs;

pub fn day(challenge: u8) {
    let contents = &fs::read_to_string("../data/3.txt")
        .expect("Should have been able to read the file");
    let borderd_content = ".".repeat(142).to_owned()+"\n."+&contents.replace("\n", ".\n.")+"\n"+&".".repeat(142);

    let lines = borderd_content.split("\n").map(|line| line.split("").filter(|c| c != &"" && c != &"\r").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();


    let mut number: String = "".to_string();
    let mut sum = 0;
    for y in 1..(lines.len()-1) {
        let mut adjacent_symbol = false;
        for x in 1..(lines[y].len()-1) {
            if lines[y][x].chars().all(char::is_numeric) {
                let adjacent_symbols_count = vec![lines[y][x-1], lines[y][x+1], lines[y-1][x], lines[y+1][x], lines[y-1][x-1], lines[y-1][x+1], lines[y+1][x-1], lines[y+1][x+1]]
                    .iter().filter(|c| !c.chars().all(char::is_numeric) && !c.eq(&&".")).count();
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
    

    println!("Day 3, challenge {}: {}", challenge, sum);
}
