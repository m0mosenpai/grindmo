use std::collections::HashMap;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap};

#[derive(PartialEq)]
#[derive(Debug)]
struct MinDistance<A, B, C>(A, B, C);

impl<A, B, C> MinDistance<A, B, C> {
    fn into_tuple(self) -> (A, B, C) {
        (self.0, self.1, self.2)
    }
}
impl<A: PartialEq, B: PartialEq, C: PartialOrd> Eq for MinDistance<A, B, C> {}
impl<A: PartialEq, B: PartialEq, C: PartialOrd> Ord for MinDistance<A, B, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.2.partial_cmp(&self.2).unwrap_or(Ordering::Equal)
    }
}
impl<A: PartialEq, B: PartialEq, C: PartialOrd> PartialOrd for MinDistance<A, B, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find(point: &usize, circuit: &Vec<usize>) -> usize {
    let mut root = circuit[*point];
    while circuit[root] != root {
        root = circuit[root];
    }
    root
}

fn union(i: &usize, j: &usize, circuit: &mut Vec<usize>) -> u64 {
    let root_i = find(i, circuit);
    let root_j = find(j, circuit);
    if circuit[root_j] == root_i {
        0
    } else {
        circuit[root_j] = root_i;
        1
    }
}

fn main() -> io::Result<()> {
    let mut pairs = 1000;
    let f = File::open("src/day_8/8.in")?;
    let reader = BufReader::new(f);
    let mut coordinates: Vec<(i64, i64, i64)> = Vec::new();
    let mut distances = BinaryHeap::new();
    let mut circuits: Vec<usize> = Vec::new();
    let mut cc: HashMap<usize, usize> = HashMap::new();
    let mut p2: u64 = 0;
    let p1: u64;

    for (i, line) in reader.lines().enumerate() {
        let coordinate = line?.split(',').map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        coordinates.push((coordinate[0], coordinate[1], coordinate[2]));
        circuits.push(i);
    }

    let mut n = coordinates.len();
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1, z1) = coordinates[i];
            let (x2, y2, z2) = coordinates[j];
            let dist = (((x2 - x1).pow(2) + (y2 - y1).pow(2) + (z2 - z1).pow(2)) as f64).sqrt();
            distances.push(MinDistance(i, j, dist));
        }
    }

    while !distances.is_empty() && pairs > 0 {
        let (i, j, _) = distances.pop().unwrap().into_tuple();
        n -= union(&i, &j, &mut circuits) as usize;
        pairs -= 1;
    }

    for point in &circuits {
        *cc.entry(find(point, &circuits)).or_insert(0) += 1;
    }
    let mut sizes: Vec<_> = cc.values().collect();
    sizes.sort_unstable_by_key(|&v| Reverse(v));
    p1 = sizes[0..3].iter().map(|v| **v as u64).product();
    println!("Part-1: {}", p1);

    while !distances.is_empty() && n > 0 {
        let (i, j, _) = distances.pop().unwrap().into_tuple();
        n -= union(&i, &j, &mut circuits) as usize;
        if n == 1 {
            p2 = (coordinates[i].0 * coordinates[j].0) as u64;
            break;
        }
    }
    println!("Part-2: {}", p2);
    Ok(())
}
