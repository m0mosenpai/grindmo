use std::fs::read_to_string;
use std::io;

fn merge_intervals(intervals: &Vec<&str>) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for interval in intervals {
        ranges.push(interval.split_once("-").map(|(l, r)| (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap())).unwrap());
    }
    ranges.sort_by_key(|&(start, _)| start);

    let mut merged: Vec<(u64, u64)> = Vec::new();
    merged.push(ranges[0]);
    for i in 1..ranges.len() {
        if merged.last().unwrap().1 >= ranges[i].0 {
            merged.last_mut().unwrap().1 = std::cmp::max(merged.last().unwrap().1, ranges[i].1);
        } else {
            merged.push(ranges[i]);
        }
    }
    merged
}

fn main() -> io::Result<()> {
    let lines = read_to_string("src/day_5/5.in").unwrap();
    let sections: Vec<Vec<&str>> = lines.split("\n\n").map(|line| line.trim().split('\n').collect()).collect();
    let abs_fresh: u64;
    let mut fresh = 0;

    let ids = &sections[1];
    let merged_ranges = merge_intervals(&sections[0]);

    for id in ids {
        let id_num = id.parse::<u64>().unwrap();
        fresh += merged_ranges.iter().filter(|range| {
            let (start, end) = range;
            (start..=end).contains(&&id_num)
        }).count();
    }
    abs_fresh = merged_ranges.iter().map(|(l, r)| r - l + 1).sum();

    println!("Part-1: {}", fresh);
    println!("Part-2: {}", abs_fresh);
    Ok(())
}
