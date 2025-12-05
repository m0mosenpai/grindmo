use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let f = File::open("src/day_4/4.in")?;
    let reader = BufReader::new(f);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        grid.push(line.unwrap().chars().collect());
    }

    let n = grid.len();
    let m = grid[0].len();
    let dirs: [(i32, i32); 8] = [
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
    ];
    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    let mut paper_rolls = 0;
    let mut rolls_removed = 0;
    let mut loops = 0;

    loop {
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '.' {
                    continue;
                }

                if dirs
                    .iter()
                    .filter(|&&(x, y)| {
                        let new_i = i as i32 + x;
                        let new_j = j as i32 + y;
                        (0..n as i32).contains(&new_i)
                            && (0..m as i32).contains(&new_j)
                            && grid[new_i as usize][new_j as usize] == '@'
                    })
                    .count()
                    < 4
                {
                    to_remove.push((i, j));
                    if loops < 1 {
                        paper_rolls += 1;
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        while !to_remove.is_empty() {
            let (x, y) = to_remove.pop().unwrap();
            grid[x][y] = '.';
            rolls_removed += 1;
        }
        loops += 1;
    }

    println!("Part-1: {}", paper_rolls);
    println!("Part-2: {}", rolls_removed);
    Ok(())
}
