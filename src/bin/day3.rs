use std::io;
use std::io::BufRead;
use regex::Regex;

fn main() {
    println!("Hello, world!");
    let stdin = io::stdin();
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for line in stdin.lock().lines() {
        for cap in re.captures_iter(&line.unwrap()) {
            let match_str = cap.get(0).unwrap().as_str();
            if match_str == "don't()" {
                enabled = false;
            } else if match_str == "do()" {
                enabled = true;
            } else if enabled {
                let aa = cap.get(2).unwrap();
                let bb = cap.get(3).unwrap();
                println!("{} x {}", aa.as_str(), bb.as_str());
                let a = cap[2].parse::<i32>().unwrap();
                let b = cap[3].parse::<i32>().unwrap();
                sum += a * b;
            }
        }
    }
    println!("sum = {}", sum);
}