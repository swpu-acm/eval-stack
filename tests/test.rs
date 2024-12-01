use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let numbers: Vec<i64> = buf
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    println!("{:?}", numbers.iter().sum::<i64>());
    println!("{:?}", numbers.iter().sum::<i64>());
    Ok(())
}
