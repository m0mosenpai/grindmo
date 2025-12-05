use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug)]
struct Ins {
    dir: char,
    dist: i32,
}

fn main() -> io::Result<()> {
    let f = File::open("src/day_1/1.in")?;
    let reader = BufReader::new(f);

    let mut input: Vec<Ins> = Vec::new();
    for line in reader.lines() {
        let mut ins = Ins { dir: '\0', dist: 0 };
        let mut dist = String::new();
        for c in line?.chars() {
            if c.is_alphabetic() {
                ins.dir = c;
            } else {
                dist.push(c);
            }
        }
        ins.dist = dist.trim().parse::<i32>().unwrap();
        input.push(ins);
    }

    let mut pos: i32 = 50;
    let mut zero_lands = 0;
    let mut zero_passes = 0;
    for ins in input {
        if ins.dir == 'R' {
            // calculate passes but skip the last one if we land on a 0
            zero_passes += (pos + ins.dist) / 100;
            if (pos + ins.dist) % 100 == 0 {
                zero_passes -= 1;
            }
            pos = (pos + ins.dist) % 100;
        } else {
            if (ins.dist - pos) > 0 {
                zero_passes += 1 + (ins.dist - pos) / 100;
                // skip counting if the we start from 0
                if pos == 0 {
                    zero_passes -= 1;
                }
                // skip counting if we land on a 0
                if (ins.dist - pos) % 100 == 0 {
                    zero_passes -= 1;
                }
                pos = (100 - (ins.dist - pos) % 100) % 100;
            } else {
                pos = pos - ins.dist;
            }
        }

        if pos == 0 {
            zero_lands += 1;
        }
    }
    println!("Part-1: {}", zero_lands);
    println!("Part-2: {}", zero_lands + zero_passes);
    Ok(())
}
