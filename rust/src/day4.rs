use fixedbitset::FixedBitSet;
use std::fs;

pub fn day(challenge: u8) {
    let contents =
        fs::read_to_string("../data/4.txt").expect("Should have been able to read the file");
    
    let mut cards: Vec<(Vec<usize>, FixedBitSet)> = vec![];
    for line in contents.split("\n") {
        let (_card_number, card_numbers) = line.split_once(":").expect("Is a valid line");
        let numbers = card_numbers
            .split(" ")
            .map(|number| number.trim())
            .filter(|number| number.chars().all(char::is_numeric) && number.len() > 0)
            .map(|number| number.parse::<usize>().expect("Expected to be a valid number"))
            .collect::<Vec<usize>>();

        let mut wining_numbers_bs = FixedBitSet::new();
        wining_numbers_bs.grow(100);

        for num in 
            numbers[10..35].to_vec() {
           // numbers[5..13].to_vec() { 
            wining_numbers_bs.put(num);

        }
        
        cards.push((numbers[0..10].to_vec(), wining_numbers_bs));
        //cards.push((numbers[0..5].to_vec(), wining_numbers_bs));
    }

    let mut sum = 0;
    for index in 0..cards.len() {
        sum += if challenge == 1 {get_points_c1(&cards, index)} else {get_points_c2(&cards, index)};
    }
    println!("Day 4, challenge {}: {}", challenge, sum);
}


fn get_points_c1(cards: &Vec<(Vec<usize>, FixedBitSet)>, index: usize) -> usize {
    let mut points = 0;
    let (numbers, wining_numbers_bs) = &cards[index];
    for num in numbers {
        if wining_numbers_bs.contains(num.to_owned()) {
            points = if points == 0 { 1 } else { points * 2 };
        }
    }
    points
}

fn get_points_c2(cards: &Vec<(Vec<usize>, FixedBitSet)>, index: usize) -> usize {
    let mut matches = 0;
    let (numbers, wining_numbers_bs) = &cards[index];
    for num in numbers {
        if wining_numbers_bs.contains(num.to_owned()) {
            matches += 1;
        }
    }
    let mut c = 1;
    for m in 1..matches+1 {
        c += get_points_c2(cards, index+m)
    }
    c
}