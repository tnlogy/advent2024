use std::io;
use std::io::BufRead;
use std::time::Instant;

type Num = u128;
fn main() {
    let start = Instant::now();
    let stdin = io::stdin();
    let mut col0: Vec<Num> = Vec::new();
    let mut col1: Vec<Num> = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(data) = line {
            let mut iter = data.split_whitespace();
            col0.push(iter.next().unwrap().parse::<Num>().unwrap());
            col1.push(iter.next().unwrap().parse::<Num>().unwrap());
        }
    }

    col0.sort();
    col1.sort();

    let mut sum = 0;
    for i in 0..col0.len() {
        sum += col0[i].abs_diff(col1[i]);
    }
    println!("part1: {} {}", sum, col0.iter().zip(col1.iter()).map(|(a, b)| a.abs_diff(*b)).sum::<Num>());
    assert_eq!(sum, 2031679);

    let mut sum2 = 0;
    for i in 0..col0.len() {
        sum2 += col1.clone().into_iter().filter(|&x| x == col0[i]).count() as Num * col0[i];
    }
    println!("part2: {} {}", sum2, col0.iter().map(|x| col1.clone().into_iter().filter(|a| a == x).count() as Num * x).sum::<Num>());
    println!("Done in {:?}", start.elapsed());
}