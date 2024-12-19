use std::collections::HashMap;

fn main() {
    let (patterns, designs) = include_str!("../../indata/day19.txt").split_once("\n\n").unwrap();
    let mut pts: HashMap<&str, bool> = HashMap::new();
    patterns.split(", ").for_each(|p| { pts.insert(p, true); });

    fn solve<'a>(s: &'a str, pts: &mut HashMap<&'a str, bool>) -> bool {
        if pts.contains_key(s) {
            *pts.get(s).unwrap()
        } else {
            let v = (1..s.len()).any(|i| {
                let (a, b) = s.split_at(i);
                solve(a, pts) && solve(b, pts)
            });
            pts.insert(s, v);
            v
        }
    }
    let part1 = designs.split("\n").map(|s| solve(s, &mut pts)).filter(|s| *s).count();
    println!("Part 1: {}", part1);

    // part 2
    let mut pts: HashMap<&str, bool> = HashMap::new();
    patterns.split(", ").for_each(|p| { pts.insert(p, true); });
    let mut cache: HashMap<&str, usize> = HashMap::new();

    // check if whole word is in pattern or check prefix + solve2(rest) for all prefixes
    fn solve2<'a>(s: &'a str, pts: &mut HashMap<&'a str, bool>, cache: &mut HashMap<&'a str, usize>) -> usize {
        if cache.contains_key(s) { return cache[s]; }
        let v = (if pts.contains_key(s) { 1 } else { 0 })
            + (1..s.len()).map(|i| {
                let (a, b) = s.split_at(i);
                if pts.contains_key(a) {
                    solve2(b, pts, cache)
                } else {
                    0
                }
            }).sum::<usize>();
        cache.insert(s, v);
        v
    }

    let part2 = designs.split("\n").map(|s| solve2(s, &mut pts, &mut cache)).sum::<usize>();
    println!("Part 2: {:?}", &part2);
}