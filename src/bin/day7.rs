fn get_base_digit(value: usize, pos: usize, base: usize) -> usize {
    if pos == 0 {
        value % base
    } else {
        (value / base.pow(pos as u32)) % base
    }
}

fn main() {
    let input = include_str!("../../indata/day7.txt");
    let lines: Vec<_> = input.lines().collect();
    let part = 2; // change to run part1
    let n_ops = if part == 1 { 2_usize } else { 3_usize }; // *,+ or *,+,||
    let mut answer = 0usize;
    for line in lines {
        let (res, vs) = line.split_once(": ").unwrap();
        let v = res.parse::<usize>().unwrap();
        let values: Vec<usize> = vs.split(" ").collect::<Vec<&str>>().iter().map(|s| s.parse::<usize>().unwrap()).collect();

        // n_ops.pow(values.len() as u32) possible combinations for n operations +,*,||
        for i in 0..n_ops.pow(values.len() as u32) {
            let mut sum = values[0];
            for vi in 1..values.len() {
                // get digit vi-1 of base n-operations for the combination
                if get_base_digit(i,vi-1, n_ops) == 0 {
                    sum *= values[vi];
                } else if get_base_digit(i,vi-1, n_ops) == 1 {
                    sum += values[vi];
                } else { // only used in part2
                    sum = format!("{}{}", sum.to_string(), values[vi].to_string()).parse::<usize>().unwrap();
                }
                if sum > v { // skip if larger that searched value
                    break;
                }
            }
            if sum == v {
                answer += v;
                break;
            }
        }
    }
    println!("part{}: {}", part, answer);
}