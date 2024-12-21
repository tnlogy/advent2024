use std::collections::{HashMap, VecDeque};
use macroquad::color::{BLACK, BLUE, DARKGRAY, GREEN, WHITE};
use macroquad::prelude::{clear_background, draw_line, draw_rectangle, draw_text, next_frame, screen_width};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut grid = input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();
        Grid { grid: grid }
    }

    fn iter_pos(&self) -> impl Iterator<Item = (usize, usize, char)> + '_ {
        self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, &value)| (x, y, value))
        })
    }

    fn find(&self, c: char) -> Pos {
        let (x, y, _) = self.iter_pos().find(|(x, y, value)| *value == c).unwrap();
        Pos { x, y }
    }

    fn bfs(&self, start: Pos, end: Pos) -> HashMap<Pos, usize> {
        struct SE { p: Pos, steps: usize };
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

#[macroquad::main("Day20")]
async fn main() {
    let input = include_str!("../../indata/day20.txt");
    let mut grid = Grid::new(input);
    let (start, end) = (grid.find('S'), grid.find('E'));
    println!("start {:?} end {:?}", start, end);
    let s_to_e = grid.bfs(start.clone(), end.clone());
    let e_to_s = grid.bfs(end.clone(), start.clone());
    let best_cost = s_to_e[&end];
    println!("best cost {:?}", best_cost);
    let mut saves: HashMap<usize, usize> = HashMap::new();
    let mut part1 = 0;
    let mut add_cost = |cost| {
        if cost < best_cost {
            let save = best_cost - cost;
            if save >= 100 {
                println!("cost is {} {}", cost, save);
                part1 += 1;
            }
            saves.entry(save).and_modify(|x| *x += 1).or_insert(1);
        }
    };

    for y in 1..grid.grid.len()-1 {
        for x in 1..grid.grid[0].len()-1 {
            if grid.grid[y][x] == '#' {
                for ((dx1, dy1), (dx2, dy2)) in [
                    ((-1, 0), (1, 0)), ((0, -1), (0, 1)), ((-1, 0), (0, 1)), ((-1, 0), (0, -1)), ((0, -1), (1, 0)), ((0, 1), (1, 0))] {
                    let (x1, y1) = ((x as i32 + dx1) as usize, (y as i32 + dy1) as usize);
                    let (x2, y2) = ((x as i32 + dx2) as usize, (y as i32 + dy2) as usize);
                    if grid.grid[y1][x1] != '#' && grid.grid[y2][x2] != '#' {
                        let cost1 = s_to_e[&Pos { x: x1, y: y1 }] + e_to_s[&Pos { x: x2, y: y2 }] + 2;
                        let cost2 = e_to_s[&Pos { x: x1, y: y1 }] + s_to_e[&Pos { x: x2, y: y2 }] + 2;
                        add_cost(cost1);
                        add_cost(cost2);
                    }
                }
            }
        }
    }

    println!("saves {:?}", saves);
    println!("part1 {}", part1);

   loop {
        clear_background(BLACK);
        for y in 0..grid.grid.len() {
            for x in 0..grid.grid[0].len() {
                let color =  if grid.grid[y][x] != '#' { BLACK } else { GREEN };
                draw_rectangle(x as f32 * 10., 30.0 + y as f32 * 10., 8.0, 8.0, color);
            }
        }

        draw_text("Day 20", 20.0, 20.0, 30.0, WHITE);

        next_frame().await
    }
   // assert_eq!(saves[&8], 4);
    //assert_eq!(saves[&4], 14);
}