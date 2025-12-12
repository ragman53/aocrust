use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Santa {
    x: i32,
    y: i32,
}

impl Santa {
    fn new(x: i32, y: i32) -> Self {
        Santa { x, y }
    }
    fn move_next(&mut self, dir: char) {
        match dir {
            '>' => self.x += 1,
            '<' => self.x -= 1,
            '^' => self.y += 1,
            'v' => self.y -= 1,
            _ => (),
        }
    }
    fn deliver(input: &str) -> usize {
        let mut house: HashSet<Santa> = HashSet::new();
        let mut santa = Santa::new(0, 0);
        house.insert(santa);
        for p in input.chars() {
            santa.move_next(p);
            house.insert(santa);
        }
        house.len()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let count = Santa::deliver(&input);
    println!("Visited House Count: {:?}", count);
    Ok(())
}
