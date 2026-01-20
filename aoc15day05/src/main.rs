use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt").expect("Failed to read an input file");
    let mut counter_part1 = 0;
    let mut counter_part2 = 0;
    for line in input.lines() {
        let letters: Vec<char> = line.chars().collect();

        if letters.len() < 3 {
            continue;
        }

        //---PART1---
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
            counter_part1 += 1;
        }

        // --- PART 2 ---
        let mut has_pair_twice = false;
        let mut has_repeat_gap = false;

        for i in 0..letters.len() - 1 {
            let pair = (letters[i], letters[i + 1]);
            for j in i + 2..letters.len() - 1 {
                if (letters[j], letters[j + 1]) == pair {
                    has_pair_twice = true;
                    break;
                }
            }
            if has_pair_twice {
                break;
            }
        }

        for i in 0..letters.len() - 2 {
            if letters[i] == letters[i + 2] {
                has_repeat_gap = true;
                break;
            }
        }

        if has_pair_twice && has_repeat_gap {
            counter_part2 += 1;
        }
    }
    println!("PART1: Nice strings count: {}", counter_part1);
    println!("PART2: Nice strings count: {}", counter_part2);
    Ok(())
}

/*
 --- Part One ---
 A nice string is one with all of the following properties:

 It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
 It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
 It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.

 --- Part Two ---
 It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
 It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
 For example:

 qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj) and a letter that repeats with exactly one letter between them (zxz).
 xxyxx is nice because it has a pair that appears twice and a letter that repeats with one between, even though the letters used by each rule overlap.
 uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with a single letter between them.
 ieodomkazucvgmuy is naughty because it has a repeating letter with one between (odo), but no pair that appears twice.
 How many strings are nice under these new rules?
*/
