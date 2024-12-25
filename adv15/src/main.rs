#[derive(Debug)]
struct Map {
    vec: Vec<char>,
    width: usize,
    height: usize,

    robot_position: (usize, usize),
}

impl Map {
    fn from_string(s: &str) -> Map {
        let width = s.find('\n').unwrap();
        let mut vec: Vec<char> = s.chars().filter(|c| *c != '\n').collect();
        let height = vec.len() / width;

        let robot_index = vec.iter().position(|c| *c == '@').unwrap();
        vec[robot_index] = '.';

        Map {
            vec,
            width,
            height,
            robot_position: (robot_index % width, robot_index / width),
        }
    }
    fn rasterize(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
    fn get(&self, x: usize, y: usize) -> char {
        let i = self.rasterize(x, y);
        self.vec[i]
    }
    fn set(&mut self, x: usize, y: usize, c: char) {
        let i = self.rasterize(x, y);
        self.vec[i] = c;
    }

    fn try_move(&mut self, direction: (isize, isize)) -> bool {
        let mut check_position = self.robot_position;
        check_position.0 = (check_position.0 as isize + direction.0) as usize;
        check_position.1 = (check_position.1 as isize + direction.1) as usize;
        let mut did_push = false;
        while self.get(check_position.0, check_position.1) == 'O' {
            check_position.0 = (check_position.0 as isize + direction.0) as usize;
            check_position.1 = (check_position.1 as isize + direction.1) as usize;
            did_push = true;
        }
        if self.get(check_position.0, check_position.1) == '.' {
            if !did_push {
                return true;
            }

            self.set(
                (self.robot_position.0 as isize + direction.0) as usize,
                (self.robot_position.1 as isize + direction.1) as usize,
                '.',
            );
            self.set(check_position.0, check_position.1, 'O');
            return true;
        }
        false
    }

    fn try_move_up(&mut self) {
        if self.try_move((0, -1)) {
            self.robot_position.1 -= 1;
        }
    }

    fn try_move_right(&mut self) {
        if self.try_move((1, 0)) {
            self.robot_position.0 += 1;
        }
    }

    fn try_move_down(&mut self) {
        if self.try_move((0, 1)) {
            self.robot_position.1 += 1;
        }
    }

    fn try_move_left(&mut self) {
        if self.try_move((-1, 0)) {
            self.robot_position.0 -= 1;
        }
    }
}

struct WideMap {
    vec: Vec<char>,
    width: usize,
    height: usize,
    robot_position: (usize, usize),
}

impl WideMap {
    fn from_map(map: &Map) -> WideMap {
        let width = map.width * 2;
        let height = map.height;
        let mut vec = std::vec::Vec::with_capacity(width * height);
        let robot_position = (map.robot_position.0 * 2, map.robot_position.1);

        map.vec.iter().for_each(|c| match *c {
            'O' => {
                vec.push('[');
                vec.push(']');
            }
            '#' => {
                vec.push('#');
                vec.push('#');
            }
            _ => {
                vec.push('.');
                vec.push('.');
            }
        });

        WideMap {
            vec,
            width,
            height,
            robot_position,
        }
    }

    fn rasterize(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }
    fn get(&self, x: usize, y: usize) -> char {
        let i = self.rasterize(x, y);
        self.vec[i]
    }
    fn set(&mut self, x: usize, y: usize, c: char) {
        let i = self.rasterize(x, y);
        self.vec[i] = c;
    }
    fn can_push_up(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            '#' => false,
            '[' => self.can_push_up(x, y - 1) && self.can_push_up(x + 1, y - 1),
            ']' => self.can_push_up(x - 1, y - 1) && self.can_push_up(x, y - 1),
            _ => true,
        }
    }
    fn push_up(&mut self, x: usize, y: usize) {
        match self.get(x, y) {
            '#' => panic!("Invalid state: tried to push up on wall"),
            '[' => {
                self.push_up(x, y - 1);
                self.push_up(x + 1, y - 1);
                self.set(x, y, '.');
                self.set(x + 1, y, '.');
                self.set(x, y - 1, '[');
                self.set(x + 1, y - 1, ']');
            }
            ']' => {
                self.push_up(x - 1, y - 1);
                self.push_up(x, y - 1);
                self.set(x - 1, y, '.');
                self.set(x, y, '.');
                self.set(x - 1, y - 1, '[');
                self.set(x, y - 1, ']');
            }
            _ => (),
        }
    }
    fn can_push_down(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            '#' => false,
            '[' => self.can_push_down(x, y + 1) && self.can_push_down(x + 1, y + 1),
            ']' => self.can_push_down(x - 1, y + 1) && self.can_push_down(x, y + 1),
            _ => true,
        }
    }
    fn push_down(&mut self, x: usize, y: usize) {
        match self.get(x, y) {
            '#' => panic!("Invalid state: tried to push down on wall"),
            '[' => {
                self.push_down(x, y + 1);
                self.push_down(x + 1, y + 1);
                self.set(x, y, '.');
                self.set(x + 1, y, '.');
                self.set(x, y + 1, '[');
                self.set(x + 1, y + 1, ']');
            }
            ']' => {
                self.push_down(x - 1, y + 1);
                self.push_down(x, y + 1);
                self.set(x - 1, y, '.');
                self.set(x, y, '.');
                self.set(x - 1, y + 1, '[');
                self.set(x, y + 1, ']');
            }
            _ => (),
        }
    }
    fn can_push_left(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            '#' => false,
            ']' => self.can_push_left(x - 2, y),
            '[' => panic!("Invalid state: tried to push [ left"),
            _ => true,
        }
    }
    fn push_left(&mut self, x: usize, y: usize) {
        match self.get(x, y) {
            '#' => panic!("Invalid state: tried to push left on wall"),
            ']' => {
                self.push_left(x - 2, y);
                self.set(x, y, '.');
                self.set(x - 1, y, ']');
                self.set(x - 2, y, '[');
            }
            '[' => {
                panic!("Invalid state: tried to push [ left")
            }
            _ => (),
        }
    }
    fn can_push_right(&self, x: usize, y: usize) -> bool {
        match self.get(x, y) {
            '#' => false,
            '[' => self.can_push_right(x + 2, y),
            ']' => panic!("Invalid state: tried to push ] right"),
            _ => true,
        }
    }
    fn push_right(&mut self, x: usize, y: usize) {
        match self.get(x, y) {
            '#' => panic!("Invalid state: tried to push left on wall"),
            '[' => {
                self.push_right(x + 2, y);
                self.set(x, y, '.');
                self.set(x + 1, y, '[');
                self.set(x + 2, y, ']');
            }
            ']' => {
                panic!("Invalid state: tried to push ] right")
            }
            _ => (),
        }
    }
    fn try_move_up(&mut self) {
        let x = self.robot_position.0;
        let y = self.robot_position.1;
        let can_move_up = match self.get(x, y - 1) {
            '[' | ']' => self.can_push_up(x, y - 1),
            '#' => false,
            _ => true,
        };
        if can_move_up {
            self.push_up(x, y - 1);
            self.robot_position.1 -= 1;
        }
    }
    fn try_move_right(&mut self) {
        let x = self.robot_position.0;
        let y = self.robot_position.1;
        let can_move_right = match self.get(x + 1, y) {
            '[' | ']' => self.can_push_right(x + 1, y),
            '#' => false,
            _ => true,
        };
        if can_move_right {
            self.push_right(x + 1, y);
            self.robot_position.0 += 1;
        }
    }
    fn try_move_down(&mut self) {
        let x = self.robot_position.0;
        let y = self.robot_position.1;
        let can_move_down = match self.get(x, y + 1) {
            '[' | ']' => self.can_push_down(x, y + 1),
            '#' => false,
            _ => true,
        };
        if can_move_down {
            self.push_down(x, y + 1);
            self.robot_position.1 += 1;
        }
    }
    fn try_move_left(&mut self) {
        let x = self.robot_position.0;
        let y = self.robot_position.1;
        let can_move_left = match self.get(x - 1, y) {
            '[' | ']' => self.can_push_left(x - 1, y),
            '#' => false,
            _ => true,
        };
        if can_move_left {
            self.push_left(x - 1, y);
            self.robot_position.0 -= 1;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let now = std::time::Instant::now();

    let map_part = &input[..input.find("\n\n").unwrap()];
    let mut map = Map::from_string(map_part);

    let mut wide_map = WideMap::from_map(&map);

    let instructions_part = &input[input.find("\n\n").unwrap() + 2..];
    let instructions: Vec<char> = instructions_part.chars().collect();

    instructions.iter().for_each(|i| match *i {
        '^' => map.try_move_up(),
        '>' => map.try_move_right(),
        'v' => map.try_move_down(),
        '<' => map.try_move_left(),
        _ => (),
    });

    instructions.iter().for_each(|i| match *i {
        '^' => wide_map.try_move_up(),
        '>' => wide_map.try_move_right(),
        'v' => wide_map.try_move_down(),
        '<' => wide_map.try_move_left(),
        _ => (),
    });

    let mut sum: usize = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.get(x, y) == 'O' {
                sum += x + y * 100;
            }
        }
    }

    let mut wide_sum: usize = 0;
    for y in 0..wide_map.height {
        for x in 0..wide_map.width {
            if wide_map.get(x, y) == '[' {
                wide_sum += x + y * 100;
            }
        }
    }

    println!("Analysis took: {:?}", now.elapsed());
    println!("Part 1: {sum}");
    println!("Part 2: {wide_sum}")
}
