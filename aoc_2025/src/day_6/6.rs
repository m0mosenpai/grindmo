use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn part_1(content: &Vec<Vec<String>>) -> u64 {
    let n = content.len();
    let m = content[0].len();
    let mut result = 0;

    for j in 0..m {
        let mut total: u64;
        let op = &content[n - 1][j];
        if op == "+" {
            total = 0;
        } else {
            total = 1;
        }

        for i in 0..n - 1 {
            let num = content[i][j].trim().parse::<u64>().unwrap();
            if op == "+" {
                total += num;
            } else {
                total *= num;
            }
        }
        result += total;
    }
    result
}

fn part_2(content: &Vec<Vec<String>>) -> u64 {
    let n = content.len();
    let m = content[0].len();
    let mut nums: Vec<Vec<String>> = Vec::new();
    let mut ops: Vec<u64> = Vec::new();
    let mut result = 0;

    for j in 0..m {
        let op = &content[n - 1][j];
        if op == "+" {
            ops.push(0);
        } else {
            ops.push(1);
        }

        let mut tmp = Vec::new();
        for i in 0..n - 1 {
            tmp.push(content[i][j].to_string());
        }
        nums.push(tmp);
    }

    for i in 0..nums.len() {
        let mut total = ops[i];
        let max_len = nums[i].iter().max_by_key(|s| s.len()).unwrap().len();
        for ptr in 0..max_len {
            let mut tmp = Vec::new();
            for num in nums[i].iter() {
                let curr_len = num.len();
                let off: i64 = curr_len as i64 - ptr as i64 - 1;
                if (0..curr_len).contains(&(off as usize)) {
                    let c = num.chars().collect::<Vec<char>>()[off as usize];
                    if !c.is_whitespace() {
                        tmp.push(c);
                    }
                }
            }
            let num = tmp.into_iter().collect::<String>().parse::<u64>().unwrap();
            if ops[i] == 0 {
                total += num;
            } else {
                total *= num;
            }
        }
        result += total;
    }
    result
}

fn main() -> io::Result<()> {
    let f = File::open("src/day_6/6.in")?;
    let reader = BufReader::new(f);
    let mut content: Vec<Vec<String>> = Vec::new();
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let last_line = &lines[lines.len() - 1];
    let ops: Vec<usize> = last_line.char_indices()
        .filter(|(_, ch)| *ch == '+' || *ch == '*')
        .map(|(i, _) | i)
        .collect();

    for (line_idx, line) in lines.iter().enumerate() {
        let nums: Vec<String> = ops.iter()
          .enumerate()
          .map(|(idx, &start)| {
              let end = ops.get(idx + 1).copied().unwrap_or(line.len());
              let mut s = line.get(start..end).unwrap_or("").to_string();
              if line_idx == lines.len() - 1 {
                  s = s.trim().to_string();
              } else {
                  if idx < ops.len() - 1 && s.ends_with(' ') {
                      s.pop();
                  }
              }
              s
          })
          .collect();
        content.push(nums);
    }
    
    println!("Part-1: {}", part_1(&content));
    println!("Part-2: {}", part_2(&content));
    Ok(())
}
