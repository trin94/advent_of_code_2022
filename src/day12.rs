use std::collections::{HashSet, VecDeque};
use std::ffi::c_uint;
use std::thread::current;

use crate::files;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Item {
    character: char,
    column: usize,
    row: usize,
}

impl Item {
    pub fn new(character: char, column: usize, row: usize) -> Self {
        Item { character, column, row }
    }

    pub fn elevation(&self) -> usize {
        match self.character {
            'a' | 'S' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            'i' => 8,
            'j' => 9,
            'k' => 10,
            'l' => 11,
            'm' => 12,
            'n' => 13,
            'o' => 14,
            'p' => 15,
            'q' => 16,
            'r' => 17,
            's' => 18,
            't' => 19,
            'u' => 20,
            'v' => 21,
            'w' => 22,
            'x' => 23,
            'y' => 24,
            'z' | 'E' => 25,
            _ => panic!()
        }
    }

    pub fn can_visit(&self, other: &Item) -> bool {
        let my = self.elevation() as i32;
        let their = other.elevation() as i32;
        their - my <= 1
    }
}

struct Map {
    width: usize,
    height: usize,
    elements: Vec<Item>,
}

impl Map {
    pub fn from(string: &String) -> Self {
        let width = string.find("\n").unwrap();
        let height = string.trim().len() / width;
        let string = string.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();
        let mut elements: Vec<Item> = vec![];
        for row in 0..height {
            for column in 0..width {
                let index: usize = column + row * width;
                let char = string.as_bytes()[index] as char;
                let position = Item::new(char, column, row);
                elements.push(position);
            }
        }
        Map { width, height, elements }
    }

    pub fn item(&self, column: usize, row: usize) -> Option<&Item> {
        let index: usize = column + row * self.width;
        self.elements.get(index)
    }

    pub fn item_at(&self, index: usize) -> Option<&Item> {
        self.elements.get(index)
    }

    pub fn item_left_of(&self, position: &Item) -> Option<&Item> {
        if position.column == 0 {
            None
        } else {
            self.item(position.column - 1, position.row)
        }
    }

    pub fn item_right_of(&self, position: &Item) -> Option<&Item> {
        if position.column >= self.width {
            None
        } else {
            self.item(position.column + 1, position.row)
        }
    }

    pub fn item_above(&self, position: &Item) -> Option<&Item> {
        if position.row == 0 {
            None
        } else {
            self.item(position.column, position.row - 1)
        }
    }

    pub fn item_below(&self, position: &Item) -> Option<&Item> {
        if position.row >= self.height {
            None
        } else {
            self.item(position.column, position.row + 1)
        }
    }

    pub fn item_neighbours_of(&self, position: &Item) -> Vec<&Item> {
        let neighbours = [
            self.item_left_of(position),
            self.item_right_of(position),
            self.item_above(position),
            self.item_below(position),
        ];
        let mut neighbours = neighbours.iter()
            .filter_map(|item| *item)
            .collect::<Vec<&Item>>();
        neighbours.sort_by(|this, that| that.elevation().cmp(&this.elevation()));
        neighbours
    }

    pub fn find(&self, char: char) -> Option<&Item> {
        self.elements.iter().find(|pos| pos.character == char)
    }

    pub fn index(&self, item: &Item) -> usize {
        item.column + item.row * self.width
    }
}

pub fn solve() {
    let file = "resources/day12.txt";
    let string = files::parse_string_from(file).unwrap();

    let map = Map::from(&string);
    part_1(&map);
}

fn part_1(map: &Map) {
    let start = map.find('S').unwrap();
    let end = map.find('E').unwrap();

    let mut visited: HashSet<&Item> = HashSet::new();
    let mut next: VecDeque<(u32, &Item)> = VecDeque::new();
    next.push_front((0, start));

    while !next.is_empty() {
        let (distance, current) = next.pop_front().unwrap();

        if current == end {
            println!("Part 1: {}", distance);
            break;
        }

        for neighbor in map.item_neighbours_of(current) {
            if visited.contains(neighbor) {
                continue;
            }
            if current.can_visit(neighbor) {
                let distance = distance + 1;
                next.push_back((distance, neighbor));
                visited.insert(neighbor);
            }
        }
    }

}