use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let path = Path::new("aoc15day01/input.txt");
    let input = match fs::read_to_string(path) {
        Ok(input) => input,
        Err(e) => return Err(Box::new(e)),
    };

    let mut counter: i32 = 0;
    for c in input.chars() {
        match c {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => (),
        }
    }
    println!("floor is {:?}", counter);
    Ok(())
}
