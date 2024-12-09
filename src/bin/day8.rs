use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize
}
impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Pos {}
impl Hash for Pos {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}


fn main() {
    let input = include_str!("../../indata/day8.txt");
    let mut grid = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut antenna_map: HashMap<char, Vec<Pos>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            if c != '.' {
                antenna_map.entry(c).or_default().push(Pos { x, y });
            }
        }
    }

    let mut antinodes_map: HashMap<Pos, bool> = HashMap::new();
    let mut part1 = 0;
    let mut antinodes_map2: HashMap<Pos, bool> = HashMap::new();
    let mut part2 = 0;

    for (c, positions) in antenna_map {
        for p in &positions {
            for op in &positions {
                if &p != &op {
                    for m in -((grid.len() * 2) as i32)..(grid.len() * 2) as i32 {
                        let x: i32 = p.x as i32 - m as i32 * (op.x as i32 - p.x as i32);
                        let y: i32 = p.y as i32 - m as i32 * (op.y as i32 - p.y as i32);
                        if x >= 0 && y >= 0 && y < grid.len() as i32 && x < grid[0].len() as i32 {
                            let antinode = Pos { x: x as usize, y: y as usize };
                            if m == 1 && !antinodes_map.contains_key(&antinode) {
                                part1 += 1;
                                antinodes_map.insert(antinode.clone(), true);
                            }
                            if !antinodes_map2.contains_key(&antinode) {
                                part2 += 1;
                                antinodes_map2.insert(antinode.clone(), true);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}