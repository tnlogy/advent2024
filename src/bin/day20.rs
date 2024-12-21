use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Pos { x: usize, y: usize, }
struct Grid { grid: Vec<Vec<char>> }

impl Grid {
    fn new(input: &str) -> Grid {
        let grid = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
        Grid { grid }
    }

    fn iter_pos(&self) -> impl Iterator<Item = (usize, usize, char)> + '_ {
        self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, &value)| (x, y, value))
        })
    }

    fn find(&self, c: char) -> Pos {
        let (x, y, _) = self.iter_pos().find(|(_, _, value)| *value == c).unwrap();
        Pos { x, y }
    }

    fn bfs(&self, start: Pos) -> HashMap<Pos, usize> {
        struct SE { p: Pos, steps: usize }
        let mut queue: VecDeque<SE> = VecDeque::new();
        let mut visited: HashMap<Pos, usize> = HashMap::new();

        queue.push_back(SE {p: start.clone(), steps: 0});
        while let Some(SE { p, steps }) = queue.pop_front() {
            visited.insert(p.clone(), steps);
            //if p.x == end.x && p.y == end.y { break; }
            for (dx, dy) in [(0i32, -1i32), (-1, 0), (1, 0), (0, 1)] {
                let (x, y) = (p.x as i32 + dx, p.y as i32 + dy);
                if x >= 0 && y >= 0 && x < self.grid.len() as i32 && y < self.grid[0].len() as i32 {
                    let (x, y) = (x as usize, y as usize);
                    if self.grid[y][x] != '#' && !visited.contains_key(&Pos { x, y }) {
                        queue.push_back(SE { p: Pos { x, y }, steps: steps + 1 });
                    }
                }
            }
        }
        visited
    }
}

fn main() {
    let input = include_str!("../../indata/day20.txt");
    let grid = Grid::new(input);
    let (start, end) = (grid.find('S'), grid.find('E'));
    println!("start {:?} end {:?}", start, end);
    let s_to_e = grid.bfs(start.clone());
    let e_to_s = grid.bfs(end.clone());
    let best_cost = s_to_e[&end];
    println!("best cost {:?}", best_cost);

    for part in [1, 2] {
        let mut saves: HashMap<usize, usize> = HashMap::new();
        let mut res = 0;
        let mut add_cost = |cost| {
            if cost < best_cost {
                let save = best_cost - cost;
                if save >= 100 {
                    res += 1;
                    saves.entry(save).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        };

        for (x, y, c0) in grid.iter_pos() {
            let max_cheat = if part == 1 { 3 } else { 21 };
            if c0 != '#' {
                for (x2, y2, c) in grid.iter_pos() {
                    let manhattan_distance: usize = ((x2 as isize - x as isize).abs() + (y2 as isize - y as isize).abs()) as usize;
                    if c != '#' && manhattan_distance < max_cheat {
                        let cost1 = s_to_e[&Pos { x, y }] + e_to_s[&Pos { x: x2, y: y2 }] + manhattan_distance;
                        let cost2 = e_to_s[&Pos { x, y }] + s_to_e[&Pos { x: x2, y: y2 }] + manhattan_distance;
                        add_cost(cost1);
                        add_cost(cost2);
                    }
                }
            }
        }
        println!("part{part} {}", res/2);
    }
}