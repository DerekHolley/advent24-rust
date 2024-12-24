use std::io::{stdin,Read};

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Robot {
    fn tick(&mut self) {
        self.position.0 += self.velocity.0;
        while self.position.0 < 0 {
            self.position.0 += WIDTH;
        }
        while self.position.0 >= WIDTH {
            self.position.0 -= WIDTH;
        }
        self.position.1 += self.velocity.1;
        while self.position.1 < 0 {
            self.position.1 += HEIGHT;
        }
        while self.position.1 >= HEIGHT {
            self.position.1 -= HEIGHT;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let pos_x = line[2..line.find(',').unwrap()].parse().unwrap();
            let pos_y = line[line.find(',').unwrap() + 1..line.find(' ').unwrap()]
                .parse()
                .unwrap();
            let vel_part = &line[line.find(' ').unwrap() + 1..];
            let vel_x = vel_part[2..vel_part.find(',').unwrap()].parse().unwrap();
            let vel_y = vel_part[vel_part.find(',').unwrap() + 1..vel_part.len()]
                .parse()
                .unwrap();

            Robot {
                position: (pos_x, pos_y),
                velocity: (vel_x, vel_y),
            }
        })
        .collect();

    



    let safety_factor = analyze_part_1(&robots);

    println!("Part 1: {safety_factor}");

    println!("Part 2: begins now");
    analyze_part_2(&robots);
}

fn analyze_part_1(input_robots:&Vec<Robot>) -> i64 {

    let mut robots = input_robots.clone();

    let num_steps = 100;
    for _ in 0..num_steps {
        robots.iter_mut().for_each(|r| r.tick());
    }

    let mut quadrant_top_left = 0;
    let mut quadrant_top_right = 0;
    let mut quadrant_bottom_left = 0;
    let mut quadrant_bottom_right = 0;

    robots.iter().for_each(|r| {
        //this comparisons are ugly, but they take care of the special case where the robot isn't in a quadrant
        if r.position.0 < WIDTH / 2 {
            if r.position.1 < HEIGHT / 2 {
                quadrant_top_left += 1;
            } else if r.position.1 > HEIGHT / 2 {
                quadrant_top_right += 1;
            }
        } else if r.position.0 > WIDTH / 2 {
            if r.position.1 < HEIGHT / 2 {
                quadrant_bottom_left += 1;
            } else if r.position.1 > HEIGHT / 2 {
                quadrant_bottom_right += 1;
            }
        }
    });

    quadrant_top_left * quadrant_top_right * quadrant_bottom_left * quadrant_bottom_right
}

fn analyze_part_2(input_robots:&Vec<Robot>) {

    let mut robots = input_robots.clone();

    for i in 0.. {
        let robot_string = stringify_robots(&robots);
        if robot_string.as_bytes().windows(10).any(|a| {
            return a.iter().all(|item| *item != b'.' && *item == a[0]);
        }) {
            //This one has potertial
            println!("{robot_string}");
            println!("Iterations: {i}");

            let _ = stdin().read(&mut [0u8]).unwrap();
        }
        robots.iter_mut().for_each(|r| r.tick());
    }
}


fn stringify_robots(robots: &Vec<Robot>) -> String {
    let mut raster:Vec<u8> = vec![0;WIDTH as usize * HEIGHT as usize];
    let rasterize = |x:i32,y:i32| { (y * WIDTH + x) as usize };
    robots.iter().for_each(|r|{
        raster[rasterize(r.position.0, r.position.1)] += 1;
    });

    let mut s = std::string::String::with_capacity(((WIDTH + 1) * HEIGHT) as usize);
    
    for y in 0..HEIGHT{
        for x in 0..WIDTH {
            let v = raster[rasterize(x,y)];
            if v == 0 {
                s.push('.');
            } else {
                s.push_str(v.to_string().as_ref());
            }
        }
        s.push('\n');
    }
    s
}