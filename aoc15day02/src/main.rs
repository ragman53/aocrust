use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Gift {
    l: u32,
    w: u32,
    h: u32,
}

impl FromStr for Gift {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dims: Vec<u32> = s
            .split("x")
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;
        dims.sort();
        if dims.len() != 3 {
            return Err("Invalid dimensions count".into());
        }
        Ok(Gift {
            l: dims[0],
            w: dims[1],
            h: dims[2],
        })
    }
}

impl Gift {
    fn paper_sum(&self) -> u32 {
        let slack: u32 = self.l * self.w;
        let surface: u32 = 2 * (self.l * self.w + self.w * self.h + self.h * self.l);
        slack + surface
    }
    fn ribbon(&self) -> u32 {
        let length: u32 = (2 * self.l + 2 * self.w) + self.l * self.w * self.h;
        length
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut paper_needed: u32 = 0;
    let mut ribbon_needed: u32 = 0;
    let input = fs::read_to_string("input.txt")?;
    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let gift: Gift = line.parse::<Gift>()?;
        println!("Line {} {:?}", i, gift);
        paper_needed += gift.paper_sum();
        ribbon_needed += gift.ribbon();
    }
    println!("Total paper: {}", paper_needed);
    println!("Total ribbon: {}", ribbon_needed);
    Ok(())
}
