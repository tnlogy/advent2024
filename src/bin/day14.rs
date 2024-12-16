use regex::Regex;

#[derive(Debug)]
struct Bot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

fn main() {
    for part in [1, 2] {
        let input = include_str!("../../indata/day14.txt");
        let re = Regex::new(r"p=(\d+),(\d+) v=([-+]*\d+),([-+]*\d+)").unwrap();
        let mut bots: Vec<Bot> = Vec::new();
        for line in input.lines() {
            if let Some(caps) = re.captures(line) {
                bots.push(Bot {
                    x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    dx: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                    dy: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                });
            }
        }

        let width = 101;
        let height = 103;
        for i in 0..(if part == 1 { 100 } else { 7051 }) {
            let mut grid: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];
            for mut bot in bots.iter_mut() {
                bot.x += bot.dx;
                bot.y += bot.dy;
                if bot.x >= width { bot.x -= width; }
                if bot.y >= height { bot.y -= height; }
                if bot.x < 0 { bot.x = width + bot.x; }
                if bot.y < 0 { bot.y = height + bot.y; }

                grid[bot.y as usize][bot.x as usize] = 'X';
            }

            if i >= 7050 { // print the pattern
                println!("visual inspection of move {}", i + 1);
                println!("{}", grid.iter()
                    .map(|row| row.iter().collect::<String>())
                    .collect::<Vec<String>>()
                    .join("\n"));
            }
        }

        let (mut s0, mut s1, mut s2, mut s3) = (0, 0, 0, 0);
        for bot in bots.iter() {
            if bot.x < width / 2 {
                if bot.y < height / 2 {
                    s0 += 1;
                } else if bot.y > height / 2 {
                    s1 += 1;
                }
            } else if bot.x > width / 2 {
                if bot.y < height / 2 {
                    s2 += 1;
                } else if bot.y > height / 2 {
                    s3 += 1;
                }
            }
        }
        println!("part{part} {:?}", s0 * s1 * s2 * s3);
    }
}