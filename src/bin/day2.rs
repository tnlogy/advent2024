use std::io;
use std::io::BufRead;

fn without(data: Vec<u32>, i: usize) -> Vec<u32> {
    data.into_iter().enumerate().filter(|&(index, _)| index != i).map(|(_, value)| value).collect()
}
fn is_safe(data: Vec<u32>, faults: usize) -> bool {
    if faults > 1 { return false; }
    let mut dec = true;
    let mut inc = true;
    for (i, (a, b)) in data.clone().into_iter().zip(data.clone().into_iter().skip(1)).enumerate() {
        let diff = a.abs_diff(b);
        if diff > 3 || diff == 0  { return is_safe(without(data.clone(), i), faults + 1) || is_safe(without(data.clone(), i+1), faults + 1); }
        if a < b { dec = false; }
        if a > b { inc = false; }
        if !dec && !inc { return is_safe(without(data.clone(), i), faults + 1) || is_safe(without(data.clone(), i+1), faults + 1); }
    }
    true
}

fn parse_list(str: &str) -> Vec<u32> {
    str.split_whitespace().filter_map(|x| x.parse().ok()).collect()
}
fn main() {
    let rows = io::stdin().lock().lines().filter_map(Result::ok).collect::<Vec<String>>();
    let part1 = rows.clone().into_iter().filter(|r| is_safe(parse_list(r), 1)).count();
    println!("part1: {}", part1);
    let part1 = rows.into_iter().filter(|r| is_safe(parse_list(r), 0)).count();
    println!("part2: {}", part1);
}