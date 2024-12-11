use std::collections::HashMap;

fn after_n_blinks(s: usize, n: usize, cache: &mut HashMap<(usize, usize),usize>) -> usize {
    if cache.contains_key(&(s, n)){
        return cache[&(s, n)];
    }
    let mut stones = vec![];
    if s == 0 {
        stones.push(1);
    } else {
        let s_string = format!("{}", s);
        if s_string.len() % 2 == 0 {
            let (a, b) = s_string.split_at(s_string.len() / 2);
            stones.push(a.parse::<usize>().unwrap());
            stones.push(b.parse::<usize>().unwrap());
        } else {
            stones.push(s * 2024);
        }
    }
    let res = if n == 1 {
        stones.len()
    } else {
        stones.iter().map(|s| after_n_blinks(*s, n - 1, cache)).sum::<usize>() as usize
    };
    cache.insert((s, n), res);
    res
}

fn main() {
    let input = include_str!("../../indata/day11.txt");
    let stones = input.split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
    println!("part1 {}", stones.iter().map(|s| after_n_blinks(*s, 25, &mut cache)).sum::<usize>());
    println!("part2 {}", stones.iter().map(|s| after_n_blinks(*s, 75, &mut cache)).sum::<usize>());
}