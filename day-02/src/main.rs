use regex::Regex;
use std::{fs::File, io::BufRead, io::BufReader};
use std::str::FromStr;

fn main() {
    //  12 red cubes, 13 green cubes, 14 blue cubes

    // Game 1: 5 red, 1 green, 2 blue; 2 green, 8 blue, 6 red; 8 red, 3 blue, 2 green; 6 red, 1 green, 19 blue; 1 red, 17 blue
    let _ = part1();
    let _ = part2();
}


trait LineReader {
    fn read_line(&self, line: String) -> i32;
}

struct FileReader<'r> {
    file: &'r str,
    processor: Box<dyn LineReader>
}

impl FileReader<'_> {
    fn read(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let file = File::open(self.file)?;
        let reader = BufReader::new(file);
        let mut result: i32 = 0;
        for _line in reader.lines() {
            let line = _line?;
            result += self.processor.read_line(line);
        }
        Ok(result)
    }
}



struct Day2Part1Processor {
    red_cubes: i32,
    green_cubes: i32,
    blue_cubes: i32,
}

impl Day2Part1Processor {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for Day2Part1Processor {
    fn default() -> Self {
        Self {
            red_cubes: 0,
            green_cubes: 0,
            blue_cubes: 0,
        }
    }
}

struct GameInfo<'r> {
    id: i32,
    samples:  &'r str,
}


fn read_game_info(line: &str) -> GameInfo<'_> {
    let game_re = Regex::new(r"(\d+):(.*)").unwrap();
    let groups = game_re.captures(line).unwrap();
    let game_id = groups.get(1).unwrap().as_str();
    let game_samples = groups.get(2).unwrap().as_str();

    return GameInfo { id: to_i32(game_id), samples: game_samples }
}


impl LineReader for Day2Part1Processor {
    fn read_line(&self, line: String) -> i32 {
        let game_info = read_game_info(line.as_str());


        let mut result = game_info.id;

        let samples_re = Regex::new(r"(?:(\d+) (red|green|blue))").unwrap();
        for captures in samples_re.captures_iter(game_info.samples) {
            let cube_count = captures.get(1).unwrap().as_str();
            let cube_color = captures.get(2).unwrap().as_str();

            let count = to_i32(cube_count);

            if
                (cube_color == "red" &&  count > self.red_cubes) ||
                (cube_color == "blue" && count > self.blue_cubes) ||
                (cube_color == "green" && count > self.green_cubes)
            {
                result = 0;
                break;
            }
        }

        println!("Result for Game {} is {}", game_info.id, result);

        return result;
    }
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    // let file = File::open("./data/sample-part-01.txt")?;
    // let file = File::open("./data/input.txt")?;
    // let reader = BufReader::new(file);

    let mut processor = Day2Part1Processor::new();

    processor.red_cubes = 12;
    processor.green_cubes = 13;
    processor.blue_cubes = 14;

    let file_reader = FileReader {
        file: "./data/input.txt",
        processor: Box::new(processor),
    };

    let result = file_reader.read();

    println!("Result for Part 1 of Day 2 is {}", result.unwrap());

    Ok(())
}


struct Day2Part2Processor {}

impl Day2Part2Processor {
    fn new() -> Self {
        Default::default()
    }
}

impl Default for Day2Part2Processor {
    fn default() -> Self {
        Self {}
    }
}

impl LineReader for Day2Part2Processor {
    fn read_line(&self, line: String) -> i32 {
        let game_info = read_game_info(line.as_str());


        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;

        let samples_re = Regex::new(r"(?:(\d+) (red|green|blue))").unwrap();
        for captures in samples_re.captures_iter(game_info.samples) {
            let cube_count = captures.get(1).unwrap().as_str();
            let cube_color = captures.get(2).unwrap().as_str();

            let count = to_i32(cube_count);

            if cube_color == "red" &&  count > red_cubes {
                red_cubes = count;
            } else if cube_color == "green" && count > green_cubes {
                green_cubes = count;
            } else if cube_color == "blue" && count > blue_cubes {
                blue_cubes = count;
            }
        }

        let result = red_cubes * green_cubes * blue_cubes;

        println!("Result for Game {} is {}", game_info.id, result);

        return result;
    }
}


fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let processor = Day2Part2Processor::new();

    let file_reader = FileReader {
        file: "./data/input.txt",
        processor: Box::new(processor),
    };

    let result = file_reader.read();

    println!("Result for Part 1 of Day 2 is {}", result.unwrap());

    Ok(())
}
fn to_i32(str: &str) -> i32 {
    return  i32::from_str(str).unwrap();
}