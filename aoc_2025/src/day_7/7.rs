use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("src/day_7/7.in")?;
    let reader = BufReader::new(f);
    let mut lines: Vec<Vec<(char, u64)>> = Vec::new();

    for line in reader.lines() {
        let chars: Vec<(char, u64)> = line?.chars().into_iter().map(|c| (c, 0)).collect();
        lines.push(chars);
    }

    let n = lines.len();
    let m = lines[0].len();
    let mut splits = 0;
    let timelines: u64;

    for i in 0..n as i32 {
        for j in 0..m  as i32 {
            if lines[i as usize][j as usize].0 == 'S' {
                lines[i as usize][j as usize].1 = 1;
                continue;
            }

            if (lines[i as usize][j as usize].0 == '.' || lines[i as usize][j as usize].0 == '|')
                && (i - 1 >= 0 && (lines[(i - 1) as usize][j as usize].0 == 'S' || lines[(i - 1) as usize][j as usize].0 == '|')) {
                lines[i as usize][j as usize].0 = '|';
                lines[i as usize][j as usize].1 += lines[(i - 1) as usize][j as usize].1;
            } else {
                if i - 1 >= 0 && lines[(i - 1) as usize][j as usize].0 == '|' {
                    splits += 1;
                    if j - 1 >= 0 {
                        lines[i as usize][(j - 1) as usize].0 = '|';
                        lines[i as usize][(j - 1) as usize].1 += lines[(i - 1) as usize][j as usize].1;
                    }

                    if j + 1 < m as i32 {
                        lines[i as usize][(j + 1) as usize].0 = '|';
                        lines[i as usize][(j + 1) as usize].1 += lines[(i - 1) as usize][j as usize].1;
                    }
                }
            }
        }
    }
    timelines = lines.last().unwrap().iter().map(|(_, val)| val).sum();

    println!("Part-1: {}", splits);
    println!("Part-2: {}", timelines);
    Ok(())
}
