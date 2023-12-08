use std::fs;

pub fn day(challenge: u8) {
    let contents =
        fs::read_to_string("../data/2.txt").expect("Should have been able to read the file");

    let result: i128;
    if challenge == 1 {
        result = contents
            .split("\n")
            .map(|line| {
                let line = line.split(":").collect::<Vec<&str>>();
                let game_number = line[0]
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .get(1)
                    .expect("Game Number not Present")
                    .parse::<i128>()
                    .expect("Should have been able to parse the game number");
                let rounds = line[1].split(";").all(|round| {
                    round.split(",").all(|turn| {
                        let turn = turn.split_whitespace().collect::<Vec<&str>>();
                        let block_count = turn[0]
                            .parse::<i128>()
                            .expect("Should have been able to parse the block count");
                        if turn[1] == "red" && block_count > 12
                            || turn[1] == "green" && block_count > 13
                            || turn[1] == "blue" && block_count > 14
                        {
                            return false;
                        }
                        return true;
                    })
                });
                if rounds {
                    return game_number;
                }
                return 0;
            })
            .sum();
    } else {
        result = contents
            .split("\n")
            .map(|line| {
                let line = line.split(":").collect::<Vec<&str>>();
                let mut biggest_red = 0;
                let mut biggest_green = 0;
                let mut biggest_blue = 0;
                for turn in line[1].split(";").flat_map(|round: &str| round.split(",")) {
                    let turn = turn.split_whitespace().collect::<Vec<&str>>();
                    let block_count = turn[0]
                        .parse::<i128>()
                        .expect("Should have been able to parse the block count");
                    if turn[1] == "red" && block_count > biggest_red {
                        biggest_red = block_count;
                    } else if turn[1] == "green" && block_count > biggest_green {
                        biggest_green = block_count;
                    } else if turn[1] == "blue" && block_count > biggest_blue {
                        biggest_blue = block_count;
                    }
                }
                return biggest_red * biggest_green * biggest_blue;
            })
            .sum();
    }

    println!("Day 2, challenge {}: {}", challenge, result);
}
