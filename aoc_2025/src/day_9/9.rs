use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

fn draw_boundary(coordinates: &Vec<(i64, i64)>, grid: &mut Vec<Vec<i64>>, x_map: &HashMap<i64, i64>, y_map: &HashMap<i64, i64>) {
    let mut last: &(i64, i64);
    for (i, c) in coordinates.iter().enumerate() {
        let x = x_map[&c.0];
        let y = y_map[&c.1];
        grid[x as usize][y as usize] = 1;
        if i == 0 {
            last = coordinates.last().unwrap();
        } else {
            last = &coordinates[i - 1];
        }

        let prev_x = x_map[&last.0];
        let prev_y = y_map[&last.1];
        if x == prev_x {
            let (start, end) = (y.min(prev_y), y.max(prev_y));
            grid[x as usize][start as usize..end as usize].fill(1);
        }
        if y == prev_y {
            let (start, end) = (x.min(prev_x), x.max(prev_x));
            for row_idx in start..=end {
                grid[row_idx as usize][y as usize] = 1;
            }
        }
    }
}

fn fill(h: usize, w: usize, grid: &mut Vec<Vec<i64>>) {
    let mut to_fill: Vec<(usize, usize)> = Vec::new();
    for i in 0..h {
        let mut valid = false;
        for j in 0..w {
            if grid[i][j] == 1 && (i as i32 - 1 >= 0 && grid[i - 1][j] == 1) {
                valid = !valid;
            }
            if valid {
                to_fill.push((i, j));
            }
        }
    }
    for c in &to_fill {
        let (i, j) = *c;
        grid[i][j] = 1;
    }
}

fn calculate_prefixes(h: usize, w: usize, prefixes: &mut Vec<Vec<i64>>, grid: &Vec<Vec<i64>>) {
    for i in 1..h {
        for j in 1..w {
            prefixes[i][j] = grid[i][j] + prefixes[i - 1][j] + prefixes[i][j - 1] - prefixes[i - 1][j - 1];
        }
    }
}

fn main() -> io::Result<()> {
    let f = File::open("src/day_9/9.in")?;
    let reader = BufReader::new(f);
    let mut coordinates: Vec<(i64, i64)> = Vec::new();
    let mut max_rect: u64 = 0;

    for line in reader.lines() {
        let coordinate = line?.split(',').map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        coordinates.push((coordinate[1], coordinate[0]));
    }

    let n = coordinates.len();
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = coordinates[i];
            let (x2, y2) = coordinates[j];
            let area = (((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)) as u64;
            max_rect = std::cmp::max(max_rect, area);
        }
    }
    println!("Part-1: {}", max_rect);

    let mut x_vals: Vec<i64> = coordinates.iter().map(|c| c.0).collect();
    let mut y_vals: Vec<i64> = coordinates.iter().map(|c| c.1).collect();
    let mut x_map: HashMap<i64, i64> = HashMap::new();
    let mut y_map: HashMap<i64, i64> = HashMap::new();
    x_vals.sort();
    y_vals.sort();
    for i in 0..x_vals.len() {
        x_map.insert(x_vals[i], i as i64);
        y_map.insert(y_vals[i], i as i64);
    }
    let h = x_vals.len();
    let w = y_vals.len();
    let mut grid: Vec<Vec<i64>> = (0..h).map(|_| { vec![0; w as usize] }).collect();
    let mut prefixes: Vec<Vec<i64>> = (0..h).map(|_| { vec![0; w as usize] }).collect();

    draw_boundary(&coordinates, &mut grid, &x_map, &y_map);
    fill(h, w, &mut grid);
    calculate_prefixes(h, w, &mut prefixes, &grid);

    max_rect = 0;
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = coordinates[i];
            let (x2, y2) = coordinates[j];
            let (comp_x1, comp_y1) = (x_map[&x1], y_map[&y1]);
            let (comp_x2, comp_y2) = (x_map[&x2], y_map[&y2]);
            let (r1, c1) = (x_map[&std::cmp::min(x1, x2)], y_map[&std::cmp::min(y1, y2)]);
            let (r2, c2) = (x_map[&std::cmp::max(x1, x2)], y_map[&std::cmp::max(y1, y2)]);
            let area = (((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1)) as u64;
            let comp_area = (((comp_x1 - comp_x2).abs() + 1) * ((comp_y1 - comp_y2).abs() + 1)) as u64;
            let prefix_sum = prefixes[r2 as usize][c2 as usize]
                - prefixes[(r1 - 1) as usize][c2 as usize]
                - prefixes[r2 as usize][(c1 - 1) as usize]
                + prefixes[(r1 - 1) as usize][(c1 - 1) as usize];

            if prefix_sum == comp_area as i64 {
                max_rect = std::cmp::max(max_rect, area);
            }
        }
    }

    println!("Part-2: {}", max_rect);
    Ok(())
}
