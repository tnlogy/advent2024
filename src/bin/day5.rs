use std::collections::HashMap;

fn main() {
    let input_map = include_str!("../../indata/day5_map.txt");
    let input = include_str!("../../indata/day5.txt");
    let mut before_me_map: HashMap<&str, HashMap<&str, bool>> = HashMap::new();

    for line in input_map.lines() {
        let (a, b) = line.split_once("|").unwrap();
        if let Some(map) = before_me_map.get_mut(b) {
            map.insert(a, true);
        } else {
            let mut map: HashMap<&str, bool>= HashMap::new();
            map.insert(a, true);
            before_me_map.insert(b, map);
        }
    }

    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;

    for line in input.lines() {
        let ovalues = line.split(",").collect::<Vec<&str>>();
        let mut values = ovalues.clone();
        let mut fixed = false;
        loop {
            let mut ok = true;
            for i in 0..values.len() {
                if let Some(map) = before_me_map.get(&values[i]) {
                    for j in i + 1..values.len() {
                        if let Some(_) = map.get(&values[j]) {
                            ok = false;
                            fixed = true;
                            values.swap(i, j);
                            break;
                        }
                    }
                }
                if !ok { break; }
            }
            if ok { break; }
        }
        let a = values[(values.len() / 2) as usize].parse::<u32>().expect("not a number");
        if fixed {
            sum_part2 += a;
        } else {
            sum_part1 += a;
        }
    }
    println!("part1: {}", sum_part1);
    println!("part2: {}", sum_part2);
}