use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let secret_key = "iwrupvqb";

    for i in 1..=u64::MAX {
        let input = format!("{}{}", secret_key, i);
        let digest = md5::compute(input);
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            println!("Answer: {}", i);
            println!("Hash: {:x}", digest);
            break;
        }
    }

    Ok(())
}
