use std::env;
mod day1;
mod day2;
mod day3;
mod day4;


fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    let day = args[1].parse::<usize>().expect("Please provide a valid day number");
    match day {
      1 => {
        day1::day(1);
        day1::day(2);
      },
      2 => {
        day2::day(1);
        day2::day(2);
      },
      3 => {
        day3::day(1);
        day3::day(2);
        print!("challenge 2 is not working yet")
      },
      4 => {
        day4::day(1);
        day4::day(2);
      },
      _ => {
        println!("Day {} not implemented yet", day);
      }
    }
  } else {
    print!("Please provide a day number")
  }
}