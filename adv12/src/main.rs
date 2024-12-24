struct Map {
    vec: Vec<char>,
    explored: Vec<bool>,
    region_tracker:Vec<bool>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_string(s: &str) -> Map {
        let width = s.chars().position(|c| c == '\n').unwrap();
        let vec: Vec<char> = s.chars().filter(|c| *c != '\n').collect();
        let explored: Vec<bool> = vec![false; vec.len()];
        let region_tracker: Vec<bool> = vec![false; vec.len()];
        let height = vec.len() / width;
        Map {
            vec,
            explored,
            region_tracker,
            width,
            height,
        }
    }
    fn rasterize(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
    fn get(&self, x: usize, y: usize) -> char {
        let i = self.rasterize(x, y);
        self.vec[i]
    }
    fn is_explored(&self, x: usize, y: usize) -> bool {
        let i = self.rasterize(x, y);
        self.explored[i]
    }
    fn explore(&mut self, x: usize, y: usize) {
        let i = self.rasterize(x, y);
        self.region_tracker[i] = true;
        self.explored[i] = true;
    }
    fn is_in_region(&self, x:usize, y:usize) -> bool {
        let i = self.rasterize(x, y);
        self.region_tracker[i]
    }
    fn reset_region(&mut self) {
        self.region_tracker.fill(false);
    }

    fn count_region_walls(&mut self) -> u64 {

        let mut wall_count:u64 = 0;
        
        let mut tracking_top = false;
        let mut tracking_bottom = false;
        for y in 0..self.height {
            for x in 0..self.width {

                if !self.is_in_region(x, y){
                    if tracking_top {
                        wall_count += 1;
                        tracking_top = false;
                    }
                    if tracking_bottom {
                        wall_count += 1;
                        tracking_bottom = false;
                    }
                    continue;
                }
                if y > 0 {
                    match (tracking_top, self.is_in_region(x,y-1)){
                        (true, true) => {
                            wall_count += 1;
                            tracking_top = false;
                        }
                        (false, false) => {
                            tracking_top = true;
                        }
                        _ => ()
                    };
                } else {
                    tracking_top = true;
                }
                if y < self.height - 1 {
                    match (tracking_bottom, self.is_in_region(x,y+1)){
                        (true, true) => {
                            wall_count += 1;
                            tracking_bottom = false;
                        }
                        (false, false) => {
                            tracking_bottom = true;
                        }
                        _ => ()
                    };
                } else {
                    tracking_bottom = true;
                }
            }
            if tracking_top {
                wall_count += 1;
                tracking_top = false;
            }
            if tracking_bottom {
                wall_count += 1;
                tracking_bottom = false;
            }
        }

        let mut tracking_left = false;
        let mut tracking_right = false;
        for x in 0..self.width {
            for y in 0..self.height {
                
                if !self.is_in_region(x, y){
                    if tracking_left {
                        wall_count += 1;
                        tracking_left = false;
                    }
                    if tracking_right {
                        wall_count += 1;
                        tracking_right = false;
                    }
                    continue;
                }
                if x > 0 {
                    match (tracking_left, self.is_in_region(x-1,y)){
                        (true, true) => {
                            wall_count += 1;
                            tracking_left = false;
                        }
                        (false, false) => {
                            tracking_left = true;
                        }
                        _ => ()
                    };
                } else {
                    tracking_left = true;
                }
                if x < self.width - 1 {
                    match (tracking_right, self.is_in_region(x+1,y)){
                        (true, true) => {
                            wall_count += 1;
                            tracking_right = false;
                        }
                        (false, false) => {
                            tracking_right = true;
                        }
                        _ => ()
                    };
                } else {
                    tracking_right = true;
                }
            }
            
            if tracking_left {
                wall_count += 1;
                tracking_left = false;
            }
            if tracking_right {
                wall_count += 1;
                tracking_right = false;
            }
        }

        wall_count
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut map = Map::from_string(&input);

    let mut part_1_sum = 0;
    let mut part_2_sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.is_explored(x, y) {
                continue;
            }

            map.explore(x, y);
            let (perimeter, area) = explore_region(&mut map, x, y);
            part_1_sum += perimeter * area;
            let c = map.get(x, y);

            let wall_count = map.count_region_walls();
            part_2_sum += area * wall_count;
            println!("Region found, char:{c}, per:{perimeter}, area: {area}, walls: {wall_count}");

            map.reset_region();
        }
    }

    println!("Part 1 sum of region costs: {part_1_sum}");
    println!("Part 2 sum of region costs: {part_2_sum}");
}

fn explore_region(map: &mut Map, x: usize, y: usize) -> (u64, u64) {
    let c = map.get(x, y);
    let mut connected_perimeter = 0;
    let mut connected_area = 1;

    //check left
    if x > 0 && map.get(x - 1, y) == c {
        if !map.is_explored(x - 1, y) {
            map.explore(x - 1, y);
            let (per, area) = explore_region(map, x - 1, y);
            connected_perimeter += per;
            connected_area += area;
        }
    } else {
        connected_perimeter += 1;
    }

    //check top
    if y > 0 && map.get(x, y - 1) == c {
        if !map.is_explored(x, y - 1) {
            map.explore(x, y - 1);
            let (per, area) = explore_region(map, x, y - 1);
            connected_perimeter += per;
            connected_area += area;
        }
    } else {
        connected_perimeter += 1;
    }

    //check right
    if x < map.width - 1 && map.get(x + 1, y) == c {
        if !map.is_explored(x + 1, y) {
            map.explore(x + 1, y);
            let (per, area) = explore_region(map, x + 1, y);
            connected_perimeter += per;
            connected_area += area;
        }
    } else {
        connected_perimeter += 1;
    }

    //check bottom
    if y < map.height - 1 && map.get(x, y + 1) == c {
        if !map.is_explored(x, y + 1) {
            map.explore(x, y + 1);
            let (per, area) = explore_region(map, x, y + 1);
            connected_perimeter += per;
            connected_area += area;
        }
    } else {
        connected_perimeter += 1;
    }

    (connected_perimeter, connected_area)
}
