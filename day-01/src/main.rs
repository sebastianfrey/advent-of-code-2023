use regex::Regex;
use std::str::FromStr;
use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let _ = part1();
    let _ = part2();
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    // let file = File::open("./data/sample-part-01.txt")?;
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);
    let re = Regex::new(r"\d").unwrap();

    let mut sum = 0;

    for _line in reader.lines() {
        let line = _line?;

        let mut first: &str = "";
        let mut last: &str = "";

        for _char in line.as_str().split("") {
            if re.captures(_char).is_none() {
                continue;
            }
            last = _char;
            if first == "" {
                first = last;
            }
        }
        let mut owned_first = first.to_owned();
        owned_first.push_str(last);
        let value = i32::from_str(owned_first.as_str()).unwrap();
        sum += value;
    }

    println!("Result for Part 1 of Day 01 is {}", sum);

    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    // let file = File::open("./data/sample-part-02.txt")?;
    let file = File::open("./data/input.txt")?;
    let reader = BufReader::new(file);
    let numbers = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];

    let mut sum = 0;

    for _line in reader.lines() {
        let line = _line?;

        let mut first_idx = usize::MAX;
        let mut last_idx = 0;

        let mut first = -1;
        let mut last = -1;

        for number in numbers.iter() {
            let number_first_idx = line.find(number);
            let number_last_idx = line.rfind(number);

            if number_first_idx == None && number_last_idx == None {
                continue;
            }

            let unwrapped_number_first_idx = number_first_idx.unwrap();
            if unwrapped_number_first_idx <= first_idx {
                first = to_i32(number);
                first_idx = unwrapped_number_first_idx;
            }

            let unwrapped_number_last_idx = number_last_idx.unwrap();
            if unwrapped_number_last_idx >= last_idx {
                last = to_i32(number);
                last_idx = unwrapped_number_last_idx;
            }
        }

        if first == -1 {
            last = first;
        } else if last == -1 {
            first = last;
        }

        let value = format!("{}{}", first, last);
        sum += i32::from_str(value.as_str()).unwrap();
    }

    println!("Result for Part 2 of Day 02 is {}", sum);

    Ok(())
}

fn to_i32(str: &str) -> i32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "twone" => 1,
        "oneight" => 8,
        "eighthree" => 3,
        "eightwo" => 2,

        _ => i32::from_str(str).unwrap(),
    }
}
