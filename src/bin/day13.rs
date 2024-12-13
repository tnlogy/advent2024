use regex::Regex;

fn parse_arcade(arcade: &str) -> (f64, f64, f64, f64, f64, f64) {
    let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();
    if let Some(caps) = re.captures(arcade) {
        let ax = caps[1].parse::<f64>().unwrap();
        let ay = caps[2].parse::<f64>().unwrap();
        let bx = caps[3].parse::<f64>().unwrap();
        let by = caps[4].parse::<f64>().unwrap();
        let x = caps[5].parse::<f64>().unwrap();
        let y = caps[6].parse::<f64>().unwrap();
        (ax, ay, bx, by, x, y)
    } else {
        panic!("Failed to parse arcade: {}", arcade);
    }
}

fn main() {
    let arcades = include_str!("../../indata/day13.txt").split("\n\n").collect::<Vec<_>>();
    for part in [1, 2] {
        let mut sum = 0;
        for arcade in &arcades {
            let (ax, ay, bx, by, mut c1, mut c2) = parse_arcade(arcade);
            if part == 2 {
                c1 += 10000000000000.;
                c2 += 10000000000000.;
            }
            // c1 = ax*a + bx*b
            // c2 = ay*a + by*b
            // -- find equation for b:
            // a = (c2 - by*b) / ay
            // (ax*c2 - ax*by*b) / ay + bx * b = c1
            // (ax*c2 - ax*by*b) / ay + ay * bx * b / ay = c1
            // ax*c2 - ax*by*b + ay*bx*b = c1 * ay
            // (ay*bx - ax*by) * b  = c1*ay - ax*c2
            let b = (c1*ay - ax*c2) / (ay*bx - ax*by);
            let a = (c1 - bx * b) / ax;
            if part == 1 {
                if a == a.floor() && b == b.floor() && a <= 100. && b <= 100. {
                    sum += a.floor() as i128 * 3 + b.floor() as i128;
                }
            } else {
                if a == a.floor() && b == b.floor() && a >= 0. && b >= 0. {
                    sum += a.floor() as i128 * 3 + b.floor() as i128;
                }
            }
        }
        println!("part{part}: {}", sum);
    }
}