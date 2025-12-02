use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn is_invalid(id: u64) -> u64 {
    let chars: Vec<char> = id.to_string().chars().collect();
    let n = chars.len();
    let mid = n / 2;
    if n % 2 == 1 {
        return 0;
    }

    let mut i = 0;
    let mut j = mid;
    while i < mid && j < n {
        if chars[i] != chars[j] {
            return 0;
        }
        i += 1;
        j += 1;
    }
    id
}

fn is_invalid_2(id: u64) -> u64 {
    let chars: Vec<char> = id.to_string().chars().collect();
    let n = chars.len();

    let low = 1;
    let high = n / 2;

    for n in low..=high {
        let chunks: Vec<&[char]> = chars.chunks(n).collect();
        if let Some(first) = chunks.first() {
            if chunks.iter().all(|chunk| chunk == first) {
                return id;
            }
        }
    }
    0
}

fn main() -> io::Result<()> {
    let f = File::open("src/day_2/2.in")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    reader.read_line(&mut buffer)?;
    let product_ids: Vec<&str> = buffer.trim().split(',').collect();

    let mut p1_inv_id_sum = 0;
    let mut p2_inv_id_sum = 0;
    for idr in product_ids {
        let (start, end) = idr.split_once("-")
            .map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap()))
            .unwrap();

        for id in start..=end {
            let p1 = is_invalid(id);
            let p2 = is_invalid_2(id);
            p1_inv_id_sum += p1;
            p2_inv_id_sum += p2;
        }
    }
    println!("Part-1: {}", p1_inv_id_sum);
    println!("Part-2: {}", p2_inv_id_sum);
    Ok(())
}

