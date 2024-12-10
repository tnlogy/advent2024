use std::collections::HashSet;

fn trailheads(part: usize, x: usize, y: usize, grid: &[Vec<u32>]) -> usize {
    let trailends = trailheads_inner(x, y, grid);
    if part == 1 {
        trailends.iter().collect::<HashSet<_>>().len()
    } else {
        trailends.len()
    }
}

fn trailheads_inner(x: usize, y: usize, grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let height = grid[y][x];
    if height == 9 {
        return vec![(x, y)];
    }
    let paths = [(0i32, 1i32), (1i32, 0i32), (0i32, -1i32), (-1i32, 0i32)]
        .iter().map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|&(x, y)| {
            x >= 0 && y >= 0 && y < grid.len() as i32 && x < grid[y as usize].len() as i32
        })
        .filter(|&(x, y)| grid[y as usize][x as usize] == height + 1);
    paths
        .map(|(x, y)| trailheads_inner(x as usize, y as usize, grid))
        .flatten()
        .collect::<Vec<(usize, usize)>>()
}

fn main() {
    let input = include_str!("../../indata/day10.txt");
    let grid = input.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();
    let r = &grid[..];
    for part in [1, 2] {
        let res = grid.iter().enumerate()
            .map(|(y, row)| {
                row.iter().enumerate()
                    .filter(|(x, height)| **height == 0u32)
                    .map(move |(x, _)| trailheads(part, x, y, r))
            }).flatten().sum::<usize>();
        println!("part{}: {:?}", part, res);
    }
}