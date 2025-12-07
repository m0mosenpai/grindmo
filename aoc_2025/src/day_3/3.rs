use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn query_sparse_table(l: usize, r: usize, st: &Vec<Vec<(u32, usize)>>) -> (u32, usize) {
    let length = r - l + 1;
    let k = length.ilog2() as usize;

    std::cmp::max_by(st[k][l], st[k][r + 1 - (1 << k)], |a, b| {
        a.0.cmp(&b.0).then(b.1.cmp(&a.1))
    })
}

fn init_sparse_table(chars: &Vec<char>) -> Vec<Vec<(u32, usize)>> {
    let n = chars.len();
    let h = n.ilog2() as usize;
    let mut st: Vec<Vec<(u32, usize)>> = Vec::with_capacity(h + 1);
    for k in 0..=h {
        let row_len = n - (1 << k) + 1;
        st.push(vec![(0, 0); row_len]);
    }

    for j in 0..n {
        st[0][j] = (chars[j].to_digit(10).unwrap(), j);
    }

    for i in 1..=h {
        for j in 0..=(n - (1 << i)) {
            st[i][j] = std::cmp::max_by(st[i - 1][j], st[i - 1][j + (1 << (i - 1))], |a, b| {
                a.0.cmp(&b.0).then(b.1.cmp(&a.1))
            })
        }
    }
    st
}

fn max_jolt_n(chars: &Vec<char>, size: usize, st: &Vec<Vec<(u32, usize)>>) -> u64 {
    let nums: Vec<u64> = chars
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as u64)
        .collect();

    let n = nums.len();
    let mut result: Vec<String> = Vec::new();
    let mut start = 0;
    let (mut max_idx, mut max_val);

    for i in 0..size {
        let end = (n as i64 - size as i64 + i as i64) as usize;
        (max_val, max_idx) = query_sparse_table(start, end, &st);
        result.push(max_val.to_string());
        start = max_idx as usize + 1;
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
        let st = init_sparse_table(&chars);
        total_jolts_p1 += max_jolt_n(&chars, 2, &st);
        total_jolts_p2 += max_jolt_n(&chars, 12, &st);
    }

    println!("Part-1: {}", total_jolts_p1);
    println!("Part-2: {}", total_jolts_p2);
    Ok(())
}
