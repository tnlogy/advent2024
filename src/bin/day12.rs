use std::collections::{HashMap, VecDeque};

fn calculate_cost(x: usize, y: usize, grid: &mut Vec<Vec<char>>, part: usize) -> i32 {
    let e = grid[y][x];
    if e == '.' || e == '#' { return 0 }
    let mut area = 0;
    let mut perim = 0;
    let mut edge_id = 1;
    let mut visited = vec![];
    let mut queue = VecDeque::from([(x, y)]);
    let mut perims: HashMap<(usize, usize, usize), i32> = HashMap::new(); // x, y, dir -> edge-id
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
                        let (delta1, delta2) = if dir < 2 { ((x, y - 1, dir), (x, y + 1, dir)) } else { ((x - 1, y, dir), (x + 1, y, dir)) };
                        let e1 = *perims.get(&delta1).unwrap_or(&-1);
                        let e2 = *perims.get(&delta2).unwrap_or(&-1);
                        if e1 == -1 && e2 == -1 { // new edge
                            perim += 1;
                            edge_id += 1;
                            perims.insert((x, y, dir), edge_id);
                        } else {
                            // border case when two edges meet with different edge ids.
                            if e1 != -1 && e2 != -1 && e1 != e2 { perim -= 1; }
                            perims.insert((x, y, dir), if e1 != -1 { e1 } else { e2 });
                        }
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
    area * perim
}

fn main() {
    for part in [2] {
        let input = include_str!("../../indata/day12.txt");
        let mut grid = input
            .lines()
            .map(|l| l.chars().collect())
            .collect::<Vec<Vec<char>>>();

        // add padding around grid
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
                res += calculate_cost(x, y, &mut grid, part);
            }
        }
        println!("Part {part}: {res}");
    }
}
