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
    fn ribbon_sum(&self) -> u32 {
        let wrap: u32 = 2 * self.l + 2 * self.w;
        let bow: u32 = self.l * self.w * self.h;
        wrap + bow
    }
    fn from_input(input: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.parse::<Self>())
            .collect()
    }
    fn total_paper(gifts: &[Self]) -> u32 {
        gifts.iter().map(|g| g.paper_sum()).sum()
    }
    fn total_ribbon(gifts: &[Self]) -> u32 {
        gifts.iter().map(|g| g.ribbon_sum()).sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let gifts = Gift::from_input(&input)?;

    println!("Total paper: {}", Gift::total_paper(&gifts));
    println!("Total ribbon: {}", Gift::total_ribbon(&gifts));
    Ok(())
}
