#[derive(Debug)]
struct Map {
    vec: Vec<char>,
    nodes_p1: Vec<bool>,
    nodes_p2: Vec<bool>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(str: &str) -> Map {
        let vec: Vec<char> = str.chars().collect();
        let nodes_p1: Vec<bool> = vec![false; vec.len()];
        let nodes_p2: Vec<bool> = vec![false; vec.len()];
        let width = vec.iter().position(|c| *c == '\n').unwrap();
        let vec: Vec<char> = str.chars().collect();
        let height = vec.len() / (width + 1);
        Map {
            vec,
            nodes_p1,
            nodes_p2,
            width,
            height,
        }
    }
    fn rasterize(&self, x: usize, y: usize) -> usize {
        (self.width + 1) * y + x
    }
    fn get(&self, x: usize, y: usize) -> char {
        self.vec[self.rasterize(x, y)]
    }
    fn node_p1(&mut self, x: usize, y: usize) {
        let i = self.rasterize(x, y);
        self.nodes_p1[i] = true;
    }
    fn node_p2(&mut self, x: usize, y: usize) {
        let i = self.rasterize(x, y);
        self.nodes_p2[i] = true;
    }
    fn check(&mut self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }
}

fn main() {

    let input = std::fs::read_to_string("input.txt").unwrap();

    let now = std::time::Instant::now();

    let mut map = Map::new(input.as_ref());

    for y in 0..map.height {
        for x in 0..map.width {
            let c = map.get(x, y);
            if c == '.' {
                continue;
            };

            for dy in 0..map.height {
                for dx in 0..map.width {
                    if x == dx && y == dy {
                        continue;
                    };
                    if c == map.get(dx, dy) {
                        let delta_x = dx as isize - x as isize;
                        let delta_y = dy as isize - y as isize;
                        if map.check(x as isize + delta_x * 2, y as isize + delta_y * 2) {
                            map.node_p1(
                                (x as isize + delta_x * 2) as usize,
                                (y as isize + delta_y * 2) as usize,
                            );
                        }

                        let mut scalar = 1;
                        while map.check(x as isize + delta_x * scalar, y as isize + delta_y * scalar) {
                            map.node_p2(
                                (x as isize + delta_x * scalar) as usize,
                                (y as isize + delta_y * scalar) as usize,
                            );
                            scalar += 1;
                        }
                    }
                }
            }
        }
    }

    let part_1_total: usize = map.nodes_p1.iter().filter(|b| **b).count();
    let part_2_total: usize = map.nodes_p2.iter().filter(|b| **b).count();

    println!("Analysis took: {:?}", now.elapsed());
    
    println!("Part 1: {part_1_total}");
    println!("Part 2: {part_2_total}");
}
