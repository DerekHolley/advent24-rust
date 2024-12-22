#[derive(Debug)]
struct Map {
    vec: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_str(s: &str) -> Map {
        let width = s.chars().position(|c| c == '\n').unwrap();
        let vec: Vec<u8> = s
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let height = vec.len() / width;
        Map { vec, width, height }
    }
    fn rasterize(&self, x: usize, y: usize) -> usize {
        (self.width * y) + x
    }
    fn get(&self, x: usize, y: usize) -> u8 {
        let i = self.rasterize(x, y);
        self.vec[i]
    }
    fn set(&mut self, x: usize, y: usize, value: u8) {
        let i = self.rasterize(x, y);
        self.vec[i] = value;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let now = std::time::Instant::now();
    let mut map = Map::from_str(&input);

    let mut part_1_scores = 0;
    let mut part_2_scores = 0;

    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == 0 {
                part_2_scores += explore(&mut map, x, y, 0, true);
                part_1_scores += explore(&mut map, x, y, 0, false);
                for i in 0..map.vec.len() {
                    if map.vec[i] == 10 {
                        map.vec[i] = 9;
                    }
                }
            }
        }
    }
    println!("Analysis took: {:?}", now.elapsed());
    println!("Part 1 sum: {part_1_scores}");
    println!("Part 2 sum: {part_2_scores}");
}

fn explore(map: &mut Map, x: usize, y: usize, depth: u8, count_all_paths: bool) -> u32 {
    let map_value = map.get(x, y);

    if depth == 9 && map_value == 9 {
        if !count_all_paths {
            map.set(x, y, 10);
        }
        return 1;
    }

    if depth != map_value {
        return 0;
    }

    let mut sum_of_paths = 0_u32;
    if x > 0 {
        sum_of_paths += explore(map, x - 1, y, depth + 1, count_all_paths);
    }
    if y > 0 {
        sum_of_paths += explore(map, x, y - 1, depth + 1, count_all_paths);
    }
    if x < map.width - 1 {
        sum_of_paths += explore(map, x + 1, y, depth + 1, count_all_paths);
    }
    if y < map.height - 1 {
        sum_of_paths += explore(map, x, y + 1, depth + 1, count_all_paths);
    }

    sum_of_paths
}
