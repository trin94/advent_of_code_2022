use crate::files;

enum Shape {
    ROCK,
    PAPER,
    SCISSOR,
}

impl Shape {
    pub fn new(shape: String) -> Self {
        match shape.as_str() {
            "A" | "X" => Shape::ROCK,
            "B" | "Y" => Shape::PAPER,
            "C" | "Z" => Shape::SCISSOR,
            _ => panic!("Don't know shape '{}'", shape)
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            Shape::ROCK => 1,
            Shape::PAPER => 2,
            Shape::SCISSOR => 3,
        }
    }

    pub fn against(&self, other: &Shape) -> u32 {
        match self { // if u played self and other played other
            Shape::ROCK => match other {
                Shape::ROCK => 3,
                Shape::PAPER => 0,
                Shape::SCISSOR => 6
            },
            Shape::PAPER => match other {
                Shape::ROCK => 6,
                Shape::PAPER => 3,
                Shape::SCISSOR => 0
            },
            Shape::SCISSOR => match other {
                Shape::ROCK => 0,
                Shape::PAPER => 6,
                Shape::SCISSOR => 3
            },
        }
    }
}

struct Match {
    other_played: Shape,
    i_played: Shape,
}

impl Match {
    pub fn new(played: Shape, response: Shape) -> Self {
        return Match { other_played: played, i_played: response };
    }

    pub fn evaluate(&self) -> u32 {
        let base = self.i_played.points();
        let other = &self.other_played;
        let outcome = self.i_played.against(other);
        return outcome + base;
    }
}

pub fn solve() {
    let file = "resources/day02.txt";
    let lines = files::parse_lines_from(file);

    let as_match = |line: &String| {
        let separated: Vec<&str> = line.split(" ").collect();
        let other_played = separated.first().unwrap().to_string();
        let other_played = Shape::new(other_played);
        let i_played = separated.last().unwrap().to_string();
        let i_played = Shape::new(i_played);
        Match::new(other_played, i_played)
    };

    let sum: u32 = lines.iter()
        .map(as_match)
        .map(|m| m.evaluate())
        .sum();

    println!("Puzzle 1 sum: {}", sum)
}