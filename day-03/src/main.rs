use regex::Regex;

mod file;
mod util;

fn main() {
    let _ = part1();
    let _ = part2();
}

pub struct Day3Part1Processor {
    result: i32,
    nr_regex: Regex,
    symbol_regex: Regex,
    line_stack: Vec<String>,
}

impl Day3Part1Processor {
    pub fn new() -> Self {
        Default::default()
    }

    fn line_len(&self) -> usize {
        return self.line_stack.len();
    }

    fn first_line(&self) -> &String {
        return self.line_stack.get(0).unwrap();
    }

    fn push_line(&mut self, _line: String) -> () {
        let line = String::from(".") + &_line.to_owned() + &String::from(".");
        self.line_stack.push(line);
    }

    fn process(&mut self) -> () {
        if self.line_stack.len() <= 2 {
            return;
        }

        let previous_line = self.line_stack.get(0).unwrap();
        let current_line = self.line_stack.get(1).unwrap();
        let next_line = self.line_stack.get(2).unwrap();

        for capture in self.nr_regex.captures_iter(&current_line) {
            let nr_match = capture.get(1).unwrap();
            let nr: String = String::from(nr_match.as_str());

            let start_pos = nr_match.start();

            let previous_line_window: String =
                self.select_line_window(previous_line, &start_pos, &nr);
            let current_line_window: String =
                self.select_line_window(current_line, &start_pos, &nr);
            let next_line_window: String = self.select_line_window(next_line, &start_pos, &nr);

            let nr_window = self.lines_to_window(
                &previous_line_window,
                &current_line_window,
                &next_line_window,
            );

            if self.has_symbol(&nr_window) {
                self.result += util::to_i32(&nr);
            }
        }

        self.line_stack.remove(0);
    }

    fn select_line_window(&self, line: &String, start_pos: &usize, nr: &String) -> String {
        return line
            .chars()
            .skip(start_pos - 1)
            .take(nr.len() + 2)
            .collect();
    }

    fn lines_to_window(&self, previous: &String, current: &String, next: &String) -> String {
        return previous.to_owned() + "\n" + &current + "\n" + &next;
    }

    fn has_symbol(&self, window: &String) -> bool {
        return self.symbol_regex.captures(window.as_str()).is_some();
    }
}

impl Default for Day3Part1Processor {
    fn default() -> Self {
        Self {
            result: 0,
            nr_regex: Regex::new(r"(\d+)").unwrap(),
            symbol_regex: Regex::new(r"([^\d|.\n])").unwrap(),
            line_stack: Vec::new(),
        }
    }
}

impl file::FileProcessor for Day3Part1Processor {
    fn get_result(&mut self) -> i32 {
        return self.result;
    }

    fn on_start(&mut self) -> () {
        println!("Start processing...");
    }

    fn on_line(&mut self, line: String) -> () {
        if self.line_len() == 0 {
            self.push_line(String::from(".").repeat(line.len()))
        }
        self.push_line(line);
        self.process();
    }

    fn on_finish(&mut self) -> () {
        if self.line_len() == 2 {
            self.push_line(String::from(".").repeat(self.first_line().len()));
            self.process()
        }

        println!("Finished processing...");
    }
}

fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let processor = Day3Part1Processor::new();

    let mut file_reader = file::FileReader {
        file: "./data/input.txt",
        processor: Box::new(processor),
    };

    let _ = file_reader.read();

    let result = file_reader.processor.get_result();

    println!("Result for Part 1 of Day 3 is {}", result);

    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let processor = Day3Part2Processor::new();

    let mut file_reader = file::FileReader {
        file: "./data/sample.txt",
        processor: Box::new(processor),
    };

    let _ = file_reader.read();

    let result = file_reader.processor.get_result();

    println!("Result for Part 1 of Day 3 is {}", result);

    Ok(())
}

struct Day3Part2Processor {
    result: i32,
    gear_regex: Regex,
    nr_regex: Regex,
    line_stack: Vec<String>,
}

impl Day3Part2Processor {
    fn new() -> Self {
        Default::default()
    }

    fn line_len(&self) -> usize {
        return self.line_stack.len();
    }

    fn first_line(&self) -> &String {
        return self.line_stack.get(0).unwrap();
    }

    fn push_line(&mut self, _line: String) -> () {
        let line = String::from(".") + &_line.to_owned() + &String::from(".");
        self.line_stack.push(line);
    }

    fn process(&mut self) -> () {
        if self.line_stack.len() <= 2 {
            return;
        }

        let previous_line = self.line_stack.get(0).unwrap();
        let current_line = self.line_stack.get(1).unwrap();
        let next_line = self.line_stack.get(2).unwrap();

        for capture in self.gear_regex.captures_iter(&current_line) {
            let gear_match = capture.get(1).unwrap();
            let gear: String = String::from(gear_match.as_str());

            let start_pos = gear_match.start();

            let previous_line_window: String =
                self.select_line_window(previous_line, &start_pos, &gear);
            let current_line_window: String =
                self.select_line_window(current_line, &start_pos, &gear);
            let next_line_window: String = self.select_line_window(next_line, &start_pos, &gear);

            let gear_window = self.lines_to_window(
                &previous_line_window,
                &current_line_window,
                &next_line_window,
            );

            let previous_has_number = self.has_number(&previous_line_window);
            let current_has_number = self.has_number(&current_line_window);
            let next_has_number = self.has_number(&next_line_window);

            if !previous_has_number && !current_has_number && !next_has_number {
                return;
            }

            if previous_has_number {
                let capture = self.nr_regex.captures(&previous_line_window).unwrap();
                let nr = capture.get(1).unwrap().as_str();
                let start = capture.get(1).unwrap().start();
                println!("previous capture={}, start={}", nr, (start_pos as i32) + (start as i32) - 1);
            }

            if current_has_number {
                let capture = self.nr_regex.captures(&current_line_window).unwrap();
                let nr = capture.get(1).unwrap().as_str();
                let start = capture.get(1).unwrap().start();
                println!("current capture={}, start={}", nr, (start_pos as i32) + (start as i32) - 1);
            }

            if next_has_number {
                let capture = self.nr_regex.captures(&next_line_window).unwrap();
                let nr = capture.get(1).unwrap().as_str();
                let start = capture.get(1).unwrap().start();
                let chars: Vec<char> = next_line.chars().collect();
                let nr_idx = ((start_pos as i32) + (start as i32) - 1) as usize;
                let mut nr_pos = chars.get(nr_idx).unwrap().to_string();
                println!("next nr={}", nr_pos);

            }

            println!("{}\nstart_ps={}\n", gear_window, start_pos);
        }

        self.line_stack.remove(0);
    }

    fn select_line_window(&self, line: &String, start_pos: &usize, nr: &String) -> String {
        return line
            .chars()
            .skip(start_pos - 1)
            .take(nr.len() + 2)
            .collect();
    }

    fn lines_to_window(&self, previous: &String, current: &String, next: &String) -> String {
        return previous.to_owned() + "\n" + &current + "\n" + &next;
    }

    fn has_number(&self, line: &String) -> bool {
        self.nr_regex.captures(&line.as_str()).is_some()
    }
}

impl Default for Day3Part2Processor {
    fn default() -> Self {
        Self {
            result: 0,
            nr_regex: Regex::new(r"(\d+)").unwrap(),
            gear_regex: Regex::new(r"(\*)").unwrap(),
            line_stack: Vec::new(),
        }
    }
}

impl file::FileProcessor for Day3Part2Processor {
    fn get_result(&mut self) -> i32 {
        return self.result;
    }

    fn on_start(&mut self) -> () {
        println!("[Day3Part2Processor] Start processing...");
    }

    fn on_line(&mut self, line: String) -> () {
        if self.line_len() == 0 {
            self.push_line(String::from(".").repeat(line.len()))
        }
        self.push_line(line);
        self.process();
    }

    fn on_finish(&mut self) -> () {
        if self.line_len() == 2 {
            self.push_line(String::from(".").repeat(self.first_line().len()));
            self.process()
        }

        println!("[Day3Part2Processor] Finished processing...");
    }
}
