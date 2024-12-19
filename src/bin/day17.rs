use regex::Regex;
use rayon::prelude::*;

fn parse_registers(arcade: &str) -> (i128, i128, i128) {
    let re = Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    if let Some(caps) = re.captures(arcade) {
        let a = caps[1].parse::<i128>().unwrap();
        let b = caps[2].parse::<i128>().unwrap();
        let c = caps[3].parse::<i128>().unwrap();
        (a, b, c)
    } else {
        panic!("Failed to parse arcade: {}", arcade);
    }
}


fn main() {
    let input = include_str!("../../indata/day17.txt");
    let (regs, code) = input.split_once("\n\n").unwrap();
    let (mut a, mut b, mut c) = parse_registers(regs);
    let program = code[9..].split(",").map(|l| l.parse().unwrap()).collect::<Vec<i32>>();
    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("program = {:?}", program);



    let index = (100000000000000i128..100000000000000000i128).into_par_iter()
        .find_any(|av|{
            let (mut a, mut b, mut c) = (*av as i128, 0i128, 0i128);
        // a = *av; b = 0; c = 0;
        if av % 10000000 == 0 {
            println!("av {av}");
        }
        let mut outputs: Vec<i32> = vec![];
        let mut ip = 0;
        let mut ops = 0;
        while ip < program.len() - 1 {
            if ops > 1000000000 {
                println!("breaking {av}");
                break;
            }
            let (op, operand) = (program[ip], program[ip + 1]);
            let combo = match operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => a,
                5 => b,
                6 => c,
                _ => operand as i128
            };
            match op {
                0 => { a = a / 2i128.pow(combo as u32) }
                1 => { b = b ^ operand as i128 }
                2 => { b = combo % 8 }
                3 => { if a != 0 { ip = operand as usize; } }
                4 => { b = b ^ c }
                5 => {
                    outputs.push((combo % 8) as i32);
                    if outputs != program[..outputs.len()] {
                        return false;
                    }
                }
                6 => { b = a / 2i128.pow(combo as u32) }
                7 => { c = a / 2i128.pow(combo as u32) }
                _ => panic!("invalid op")
            }
            if a == 0 || op != 3 { ip += 2; }
            ops += 1
           // println!("{op},{operand} ip = {ip} a = {a} b = {b} c = {c}");
        }
        //println!("outputs = {:?}", outputs.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(","));
        // outputs = vec![2, 4, 1, 7, 7, 5, 0, 3, 4, 0, 1, 7, 5, 5, 3, 0];
        if outputs == program {
            println!("equal at {av}");
            return true;
        }
        false
    }).unwrap();
    println!("part 2 = {}", index);
}