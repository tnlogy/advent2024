use std::io;
use std::io::Read;
use regex::Regex;
fn transpose(input: &str) -> String {
    let lines: Vec<_> = input.lines().collect();
    let len = lines[lines.len()-1].len();
    let mut transposed = vec![String::new(); len];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            transposed[i].push(c);
        }
    }
    transposed.join("\n")
}

fn offset(input: &str, reverse: bool) -> String {
    let lines: Vec<_> = input.lines().collect();
    let len = lines[0].len();
    let mut offsets = vec![String::new(); len+1];
    for (il, line) in lines.iter().enumerate() {
        add_dots(&mut offsets[il], if reverse { len-il-1 } else {il});
        for (i, c) in line.chars().enumerate() {
            offsets[il].push(c);
        }
        add_dots(&mut offsets[il], if !reverse { len-il-1 } else {il});
    }
    offsets.join("\n")
}

fn add_dots(input: &mut String, number: usize) {
    for x in 0..number {
        input.push('.');
    }
}

fn mas(input: &str) -> u32 {
    let mut sum = 0;
    let lines = input.lines().map(|s| s.as_bytes()).collect::<Vec<_>>();
    for y in 0..lines.len()-2 {
        for x in 0..lines[y].len()-2 {
            if ((lines[y][x] as char == 'M' && lines[y+1][x+1] as char == 'A' && lines[y+2][x+2] as char == 'S') ||
                (lines[y][x] as char == 'S' && lines[y+1][x+1] as char == 'A' && lines[y+2][x+2] as char == 'M')) &&
               ((lines[y][x+2] as char == 'M' && lines[y+1][x+1] as char == 'A' && lines[y+2][x] as char == 'S') ||
                (lines[y][x+2] as char == 'S' && lines[y+1][x+1] as char == 'A' && lines[y+2][x] as char == 'M')){
                sum += 1;
            }
        }
    }
    sum
}

fn main() -> io::Result<()> {
    let mut input_s = String::new();
    io::stdin().read_to_string(&mut input_s)?;
    let input = input_s.as_str();
    let re = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();

    let transposed = transpose(&input);
    let diagonal1 = transpose(&offset(input, false));
    let diagonal2 = transpose(&offset(input, true));

    let l =  [input, &transposed, &diagonal1, &diagonal2];
    let sum = l.iter().map(|s| re.find_iter(s).count()).sum::<usize>() +
                     l.iter().map(|s| re2.find_iter(s).count()).sum::<usize>();
    println!("part1 {:?}", sum);
    println!("part1 {:?}", [re, re2].iter().flat_map(|re| l.iter().map(|s| re.find_iter(s).count())).sum::<usize>());
    println!("part2 {}", mas(&input));
    Ok(())
}