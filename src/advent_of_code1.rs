mod files {
    use std::fs::File;
    use std::io;
    use std::io::{BufRead, BufReader, Lines};
    use std::path::Path;

    pub fn parse_lines_from<P>(file: P) -> Vec<String> where P: AsRef<Path> {
        let mut lines: Vec<String> = Vec::new();
        if let Ok(file_lines) = read_lines(file) {
            for line in file_lines {
                if let Ok(ip) = line {
                    lines.push(ip);
                }
            }
        }
        return lines;
    }

    fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(BufReader::new(file).lines())
    }
}

mod problem {

    pub struct Elve {
        calories: Vec<u32>,
    }

    impl Elve {
        pub fn new() -> Self {
            Elve { calories: Vec::new() }
        }

        pub fn calories(&self) -> u32 {
            return self.calories.iter().sum();
        }
    }

    pub fn parse_elves_from(lines: Vec<String>) -> Vec<Elve> {
        let mut elves = Vec::new();
        let mut current_elve = Elve::new();
        for line in lines {
            if line.is_empty() {
                elves.push(current_elve);
                current_elve = Elve::new();
            } else {
                let food = line.parse::<u32>().unwrap();
                current_elve.calories.push(food)
            }
        }
        return elves;
    }
}



pub fn solve() {
    let file = "resources/day_1_puzzle_1_input.txt";

    let lines = files::parse_lines_from(file);
    let mut elves = problem::parse_elves_from(lines);

    elves.sort_by(|a, b| a.calories().cmp(&b.calories()));

    let calories = elves.last().unwrap().calories();
    println!("Elve with most food {}", calories);

    let elve3 = elves.pop().unwrap();
    let elve2 = elves.pop().unwrap();
    let elve1 = elves.pop().unwrap();

    let sum = elve1.calories() + elve2.calories() + elve3.calories();

    println!("Sum of those three elves: {}", sum)
}

