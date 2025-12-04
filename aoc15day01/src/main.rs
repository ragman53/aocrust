use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let path = Path::new("aoc15day01/input.txt");
    let input = fs::read_to_string(path)?;
    let mut basement: Option<usize> = None;

    let mut counter: i32 = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => (),
        }
        if counter == -1 && basement.is_none() {
            println!("floor -1: {}", i + 1);
            basement = Some(i + 1);
        }
    }
    println!("floor is {:?}", counter);
    Ok(())
}
