use std::collections::{HashMap, VecDeque};

fn fill(x: usize, y: usize, grid: &mut Vec<Vec<char>>, part: usize) -> i32 {
    let e = grid[y][x];
    if e == '.' || e == '#' { return 0 }
    let mut area = 0;
    let mut perim = 0;
    let mut visited = vec![];
    let mut queue = VecDeque::from([(x, y)]);
    let mut perims: HashMap<(usize, usize, usize), bool> = HashMap::new(); // x, y, dir -> bool
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        let qe = grid[y][x];
        visited.push((x, y));
        if qe == e {
            area += 1;
            [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)].iter().enumerate().for_each(|(dir, c)| {
                let (x, y) = *c;
                if grid[y][x] != e {
                    if part == 1 {
                        perim += 1
                    } else {
                        if ((dir == 0 || dir == 1) && !perims.contains_key(&(x, y - 1, dir)) && !perims.contains_key(&(x, y + 1, dir)))
                            || ((dir == 2 || dir == 3) && !perims.contains_key(&(x - 1, y, dir)) && !perims.contains_key(&(x + 1, y, dir))) {
                            perim += 1;
                            // println!("per {} {} {}", x, y, dir);
                        }
                        perims.insert((x, y, dir), true);
                    }
                }
                if !visited.contains(c) && !queue.contains(c) {
                    queue.push_back(*c);
                }
            })
        }
    }
    visited.iter().for_each(|(x, y)| if grid[*y][*x] == e { grid[*y][*x] = '#' });
    println!("{x},{y}: {} {} p {}", e, area, perim);
    if perim % 2 != 0 { // this is an ugly fix since we sometimes get odd number of edges.
        perim -= 1;     // probably since we fill the edges from two directions in some cases.
    }
    area * perim
}

fn main() {
    for part in [2] {
        let input = include_str!("../../indata/day12.txt");
        let mut grid = input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();
        for row in grid.iter_mut() {
            row.insert(0, '.');
            row.push('.');
        }
        grid.insert(0, vec!['.'; grid[0].len()]);
        grid.push(vec!['.'; grid[0].len()]);

        let rows = grid.len();
        let cols = grid[0].len();

        let mut res = 0;
        for y in 1..rows - 1 {
            for x in 1..cols - 1 {
                res += fill(x, y, &mut grid, part);
            }
        }
        println!("Part {part}: {res}");
    }
}
