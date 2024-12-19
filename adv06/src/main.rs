#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    None,
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Map {
    map:Vec<char>,
    visits:Vec<Direction>,
    width:usize,
    height:usize,
    guard_x: usize,
    guard_y: usize,
    guard_direction: Direction,

    original_guard_x: usize,
    original_guard_y: usize,
    original_guard_direction: Direction,
}

impl Map {
    fn new(vec:Vec<char>) -> Map {
        let len = vec.len();
        let first_break = vec.iter().position(|c| *c == '\n').unwrap();
        let guard_index = vec.iter().position(|c| *c == '^').unwrap();
        return Map{
            map: vec,
            visits: vec![Direction::None;len],
            width: first_break,
            height: len / (first_break + 1),
            guard_x: guard_index % (first_break + 1),
            guard_y: guard_index / (first_break + 1),
            guard_direction: Direction::Up,
            
            original_guard_x: guard_index % (first_break + 1),
            original_guard_y: guard_index / (first_break + 1),
            original_guard_direction: Direction::Up,
        }
    }
    fn get(&self, x:usize,y:usize) -> char {
        let i = self.rasterize(x, y);
        return self.map[i];
    }
    fn get_visit(&self, x:usize,y:usize) -> Direction {
        let i = self.rasterize(x, y);
        return self.visits[i];
    }
    fn set(&mut self, x:usize,y:usize, c:char) {
        let i = self.rasterize(x, y);
        self.map[i] = c;
    }
    fn visit(&mut self) {
        let dir = self.guard_direction;
        let i = self.rasterize(self.guard_x, self.guard_y);
        self.visits[i] = dir;
    }

    fn reset(&mut self) {
        self.guard_x = self.original_guard_x;
        self.guard_y = self.original_guard_y;
        self.guard_direction = self.original_guard_direction;
        self.visits.fill(Direction::None);
    }

    fn rasterize(&self, x:usize,y:usize) -> usize {
        (self.width + 1) * y + x
    }

    // Walk represents a detection of a loop
    fn walk(&mut self) -> Option<usize>  {

        self.visit();

        loop {
            let look_at:(isize,isize) = match &self.guard_direction {
                Direction::Up => (0,-1),
                Direction::Right => (1,0),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::None => (0,0),
            };
    
            let look_x = self.guard_x as isize + look_at.0;
            if look_x < 0 || look_x >= self.width as isize {
                break;
            }
            let look_y = self.guard_y as isize + look_at.1;
            if look_y < 0 || look_y >= self.height as isize {
                break;
            }
            
            if self.guard_direction == self.get_visit(look_x as usize, look_y as usize) {
                return None;
            }
            
            if self.get(look_x as usize, look_y as usize) == '#' {
                self.guard_direction = match &self.guard_direction {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::None => Direction::None,
                };
                continue;
            }
            self.guard_x = look_x as usize;
            self.guard_y = look_y as usize;
            self.visit();
        };

        return Some(self.visits.iter().filter(|v| **v != Direction::None).count());
    }

}

fn main() {
    let input = match std::fs::read_to_string("input.txt"){
        Ok(s) => s,
        Err(e) => panic!("Failed to read file: {}", e)
    };

    let now = std::time::Instant::now();

    let chars:Vec<char> = input.chars().collect();
    let mut map = Map::new(chars);

    let visited_positions = map.walk().unwrap();
    
    println!("Part 1: Visited {visited_positions} positions");

    let visit_check = map.visits.clone();

    let mut loop_count:usize = 0;
    for x in 0..map.width {
        for y in 0..map.height {
            // only bother trying a spot if the guard originally walked through it
            if visit_check[map.rasterize(x, y)] == Direction::None {
                continue;
            }
            map.reset();
            let original = map.get(x,y);
            map.set(x, y, '#');
            let result = map.walk();
            if result.is_none() {
                loop_count += 1;
            }
            map.set(x, y, original);
        }
    }
    
    println!("Part 2: {loop_count} positions caused a loop");

    println!("Analysis took {:?}", now.elapsed());
}
