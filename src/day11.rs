use std::collections::{BTreeMap, VecDeque};
use std::fmt;

use lazy_static::lazy_static;
use regex::Regex;

use crate::files;

struct Monkey {
    id: i32,
    items: VecDeque<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    divider: i32,
    test_passed_throw_to_id: i32,
    test_failed_throw_to_id: i32,

    inspect_counter: u32,
}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Monkey(id={},items={:?},passed={},failed={},inspected={})",
               self.id, self.items, self.test_passed_throw_to_id, self.test_failed_throw_to_id, self.inspect_counter)
    }
}

impl Monkey {
    pub fn new_from(string: &str) -> Self {
        let string = string.trim();
        let id = {
            lazy_static! { static ref RE: Regex = Regex::new(r"Monkey (?P<id>\d+):").unwrap(); }
            let captures = RE.captures(string).unwrap();
            let monkey_id: &str = &captures["id"];
            monkey_id.parse().unwrap()
        };
        let items = {
            lazy_static! { static ref RE: Regex = Regex::new(r"Starting items: (?P<items>.*?)\n").unwrap(); }
            let captures = RE.captures(string).unwrap();
            let monkey_id: &str = &captures["items"];
            monkey_id.split(",")
                .map(|id| id.trim().parse::<i32>().unwrap())
                .collect::<VecDeque<i32>>()
        };
        let operation: Box<dyn Fn(i32) -> i32> = {
            lazy_static! { static ref RE: Regex = Regex::new(r"Operation: new = old (?P<operation>[*+]) (?P<other>.*?)\n").unwrap(); }
            let captures = RE.captures(string).unwrap();
            let operation: &str = &captures["operation"];
            let other: &str = &captures["other"];
            match operation {
                "+" => {
                    let other = other.trim().parse::<i32>().unwrap();
                    Box::new(move |x: i32| x + other)
                }
                "*" => {
                    match other {
                        "old" => Box::new(|x: i32| x * x),
                        factor => {
                            let other = factor.trim().parse::<i32>().unwrap();
                            Box::new(move |old: i32| old * other)
                        }
                    }
                }
                _ => unreachable!()
            }
        };
        let divider = {
            lazy_static! { static ref RE: Regex = Regex::new(r"Test: divisible by (?P<divider>.*?)\n").unwrap(); }
            let captures = RE.captures(string).unwrap();
            let divider: &str = &captures["divider"];
            divider.trim().parse::<i32>().unwrap()
        };
        let test_passed_throw_to_id = {
            lazy_static! { static ref RE: Regex = Regex::new(r"If true: throw to monkey (?P<id>.*?)\n").unwrap(); }
            let captures = RE.captures(string).unwrap();
            let id: &str = &captures["id"];
            id.trim().parse::<i32>().unwrap()
        };
        let test_failed_throw_to_id = {
            lazy_static! { static ref RE: Regex = Regex::new(r"If false: throw to monkey (?P<id>.*?)$").unwrap(); }
            let captures = RE.captures(string).unwrap();
            let id: &str = &captures["id"];
            id.trim().parse::<i32>().unwrap()
        };
        let inspect_counter = 0;
        Monkey { id, items, operation, divider, test_passed_throw_to_id, test_failed_throw_to_id, inspect_counter }
    }

    /// Monkey inspects the item until he gets bored. Returns a tuple with new worry_level and receiver.
    pub fn inspect(&mut self, item: i32, relief_factor: i32) -> (i32, i32) {
        self.inspect_counter += 1;

        let worry_level = item;
        let worry_level: i32 = (self.operation)(worry_level);
        let worry_level: i32 = worry_level / relief_factor;

        if worry_level % self.divider == 0 {
            (worry_level, self.test_passed_throw_to_id)
        } else {
            (worry_level, self.test_failed_throw_to_id)
        }
    }
}

pub fn solve() {
    let file = "resources/day11.txt";
    let string = files::parse_string_from(file).unwrap();

    let relief_factor = 3;
    let rounds = 20;
    calculate_monkey_business(&string, relief_factor, rounds);

    // let relief_factor = 1;
    // let rounds = 10000;
    // calculate_monkey_business(&string, relief_factor, rounds);
}

fn calculate_monkey_business(string: &String, relief_factor: i32, rounds: u32) {
    let monkeys = string.split("\n\n")
        .map(|line| Monkey::new_from(line))
        .map(|monkey| (monkey.id, monkey))
        .collect::<Vec<(i32, Monkey)>>();

    let mut circus: BTreeMap<i32, Monkey> = monkeys.into_iter().collect();
    let keys = circus.keys().cloned().collect::<Vec<i32>>();

    for _ in 0..rounds {
        for key in keys.iter() {
            let monkey = circus.get_mut(&key).unwrap();
            let mut items = monkey.items.clone();
            monkey.items.clear();
            while let Some(current_item) = items.pop_front() {
                let monkey = circus.get_mut(&key).unwrap();
                let (new_item, next_owner) = monkey.inspect(current_item, relief_factor);
                let next_owner = circus.get_mut(&next_owner).unwrap();
                next_owner.items.push_back(new_item);
            }
        }
    }

    let mut inspect_counters = circus.values()
        .map(|monkey| monkey.inspect_counter)
        .collect::<Vec<u32>>();
    inspect_counters.sort();

    let monkey_business = inspect_counters.iter()
        .rev()
        .take(2)
        .fold(1, |acc, factor| acc * factor);

    println!("Monkey Business {}", monkey_business);
}
