use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt").expect("Failed to read an input file");
    let mut counter = 0;
    for line in input.lines() {
        let letters: Vec<char> = line.chars().collect();

        if letters.len() < 3 {
            continue;
        }

        let mut three_vowels = 0;
        let mut has_double = false;
        let mut has_banned = false;
        let mut prev = '\0';

        for (i, &c) in letters.iter().enumerate() {
            match c {
                'a' | 'i' | 'u' | 'e' | 'o' => three_vowels += 1,
                _ => {}
            }

            if i > 0 {
                if c == prev {
                    has_double = true;
                }
            }

            match (prev, c) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') => has_banned = true,
                _ => {}
            }

            prev = c;
        }

        if three_vowels >= 3 && has_double && !has_banned {
            counter += 1;
        }
    }
    println!("Nice strings count: {}", counter);
    Ok(())
}

/*
 A nice string is one with all of the following properties:

 It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
 It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
 It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
*/
