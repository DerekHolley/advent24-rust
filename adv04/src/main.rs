use std::time;
use std::fs;

struct Raster {
    arr:Vec<char>,
    width: usize,
}

impl Raster {
    fn check(&self, x:isize,y:isize, c:char) -> bool {
        if x < 0 || x >= self.width as isize {
            return false;
        }
        if y < 0 || (x + y * self.width as isize >= self.arr.len() as isize) {
            return false;
        }
        return self.arr[(x + y * self.width as isize) as usize] == c
    }
}

fn main () {
    let contents = match fs::read_to_string("./input.txt") {
        Ok(s) => s,
        Err(err) => panic!("Failed to read file: {}", err)
    };

    let now = time::Instant::now();
    let raster = Raster{
        arr: contents.chars().filter(|n|{*n != '\n'}).collect(),
        width: contents.find('\n').unwrap(),
    };

    let mut part_1_count:i64 = 0;
    let mut part_2_count:i64 = 0;

    let len = raster.arr.len();
    for i in 0..len {
        
        let x = i % (raster.width as usize);
        let y = i / (raster.width as usize);

        if raster.arr[i] == 'X' {
            part_1_count += search_part_1(&raster, x, y);
        }
        if raster.arr[i] == 'A' && search_part_2(&raster, x, y){
            part_2_count += 1;
        }
    }

    print!("Analysis took: {:?}\n", now.elapsed());

    print!("Part1: {part_1_count} instances\n");
    print!("Part2: {part_2_count} instances\n");
}

fn search_part_1(raster: &Raster, x:usize,y:usize) -> i64 {
    let mut sum:i64 = 0;

    if search_direction(raster, x, y, 1, 0) {
        sum += 1
    }
    if search_direction(raster, x, y, 1, 1) {
        sum += 1
    }
    if search_direction(raster, x, y, 0, 1) {
        sum += 1
    }
    if search_direction(raster, x, y, -1, 1) {
        sum += 1
    }
    if search_direction(raster, x, y, -1, 0) {
        sum += 1
    }
    if search_direction(raster, x, y, -1, -1) {
        sum += 1
    }
    if search_direction(raster, x, y, 0, -1) {
        sum += 1
    }
    if search_direction(raster, x, y, 1, -1) {
        sum += 1
    }

    return sum;
}

fn search_direction(raster: &Raster, x:usize,y:usize,dir_x:isize,dir_y:isize) -> bool {
    if !raster.check(x as isize + dir_x, y as isize + dir_y, 'M') {
        return false;
    }
    if !raster.check(x as isize + dir_x * 2, y as isize + dir_y * 2, 'A') {
        return false;
    }
    if !raster.check(x as isize + dir_x * 3, y as isize + dir_y * 3, 'S') {
        return false;
    }
    return true;
}

fn search_part_2(raster: &Raster, x:usize,y:usize) -> bool {
    if raster.check(x as isize - 1, y as isize - 1, 'M') && raster.check(x as isize + 1, y as isize - 1, 'M') && raster.check(x as isize - 1, y as isize + 1, 'S') && raster.check(x as isize + 1, y as isize + 1, 'S') {
        return true;
    }
    if raster.check(x as isize - 1, y as isize - 1, 'M') && raster.check(x as isize + 1, y as isize - 1, 'S') && raster.check(x as isize - 1, y as isize + 1, 'M') && raster.check(x as isize + 1, y as isize + 1, 'S') {
        return true;
    }
    if raster.check(x as isize - 1, y as isize - 1, 'S') && raster.check(x as isize + 1, y as isize - 1, 'S') && raster.check(x as isize - 1, y as isize + 1, 'M') && raster.check(x as isize + 1, y as isize + 1, 'M') {
        return true;
    }
    if raster.check(x as isize - 1, y as isize - 1, 'S') && raster.check(x as isize + 1, y as isize - 1, 'M') && raster.check(x as isize - 1, y as isize + 1, 'S') && raster.check(x as isize + 1, y as isize + 1, 'M') {
        return true;
    }
    return false;
}
