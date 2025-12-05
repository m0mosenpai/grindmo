use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn max_jolt_n(chars: &Vec<char>, size: usize) -> u64 {
    let nums: Vec<u64> = chars
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as u64)
        .collect();

    let n = nums.len();
    let mut result: Vec<String> = Vec::new();
    let mut start = 0;
    let (mut max_idx, mut max_val);

    for i in 0..size {
        let end = start + ((n as i64 - start as i64) - size as i64 + i as i64) as usize;
        (max_idx, max_val) = nums[start..=end]
            .iter()
            .enumerate()
            .max_by(|(idx_a, val_a), (idx_b, val_b)| {
                val_a.cmp(val_b).then_with(|| idx_b.cmp(idx_a))
            })
            .unwrap();
        result.push(max_val.to_string());
        start = start + max_idx + 1;
    }
    result.join("").parse::<u64>().unwrap()
}

fn main() -> io::Result<()> {
    let f = File::open("src/day_3/3.in")?;
    let reader = BufReader::new(f);

    let mut total_jolts_p1 = 0;
    let mut total_jolts_p2 = 0;
    for line in reader.lines() {
        let chars: Vec<char> = line.unwrap().chars().collect();
        total_jolts_p1 += max_jolt_n(&chars, 2);
        total_jolts_p2 += max_jolt_n(&chars, 12);
    }

    println!("Part-1: {}", total_jolts_p1);
    println!("Part-2: {}", total_jolts_p2);
    Ok(())
}
