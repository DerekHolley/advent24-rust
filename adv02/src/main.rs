use std::fs;

use adv02::{is_line_safe, is_line_safe_buffered};

fn main() {
    let contents = match fs::read_to_string("./input.txt"){
        Ok(file) => file,
        Err(e) => panic!("Failed to open file: {:?}", e)
    };
    
    let parts:Vec<Vec<i64>> = contents.lines().map(|line| -> Vec<i64> {
        let strings = line.split(' ');
        return strings.map(|s| -> i64{
            s.parse().unwrap()
        }).collect();
    }).collect();

    let mut safe_line_count:i64 = 0;
    let mut safe_buffered_line_count:i64 = 0;
    parts.iter().for_each(|values:&Vec<i64>| {
        if is_line_safe(values) {
            safe_line_count += 1;
        }
        if is_line_safe_buffered(values) {
            safe_buffered_line_count += 1;
        }
    });

    print!("Safe line count: {}\nSafe buffered line count: {}\n", safe_line_count, safe_buffered_line_count)
}
