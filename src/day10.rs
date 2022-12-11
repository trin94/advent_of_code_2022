use crate::files;

#[derive(Clone)]
enum Operation {
    NOOP,
    ADD(i32),
}

fn parse() -> Vec<Operation> {
    files::parse_lines_from("resources/day10.txt")
        .iter()
        .map(|line| match line.split_at(4) {
            ("noop", _) => Operation::NOOP,
            ("addx", increment) => Operation::ADD(increment.trim().parse::<i32>().unwrap()),
            (_, _) => unreachable!(),
        })
        .collect()
}

type Cycle = (i32, i32, i32);  // (cycle, 💪 during, 💪 after)

pub fn solve() {
    let operations = parse();
    let signal_history: Vec<_> = parse_signal_history(&operations);

    let mut relevant_cycles = [20, 60, 100, 140, 180, 220];
    let mut sum: i32 = 0;
    for cycle in relevant_cycles {
        let snapshot = signal_history[cycle - 1];
        let cycle = snapshot.0;
        let during = snapshot.1;
        sum += cycle * during;
    }
    println!("Sum of relevant cycles {}", sum);
}


fn parse_signal_history(operations: &Vec<Operation>) -> Vec<Cycle> {
    let mut strength: i32 = 1;
    let mut cycle: i32 = 1;
    let mut signal_history: Vec<Cycle> = Vec::new();
    for operation in operations {
        match operation {
            Operation::NOOP => {
                signal_history.push((cycle, strength, strength));
                cycle += 1;
            }
            Operation::ADD(increment) => {
                signal_history.push((cycle, strength, strength));
                cycle += 1;
                let after = strength + increment;
                signal_history.push((cycle, strength, after));
                cycle += 1;
                strength = after;
            }
        };
    }
    // for signal in &signal_history {
    //     println!("Cycle: {}, 💪 during: {}, 💪 after: {}", signal.0, signal.1, signal.2)
    // }
    signal_history
}
