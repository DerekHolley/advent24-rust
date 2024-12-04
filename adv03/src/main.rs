use std::fs;

use regex::Regex;

fn main() {
    let reg = match Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))"){
        Ok(reg) => reg,
        Err(err) => panic!("Failed to compile regex: {err}")
    };
    let contents = match fs::read_to_string("./input.txt") {
        Ok(contents) => contents,
        Err(err) => panic!("Failed to read file: {err}")
    };

    let mut sum:i64 = 0;
    let mut doing:bool = true;
    
    reg.find_iter(contents.as_ref()).for_each(|m| {
        let str = m.as_str();
        if str == "do()"{
            doing = true;
            return;
        }
        if str == "don't()" {
            doing = false;
            return;
        }
        if !doing {
            return;
        }
        let parts:Vec<&str> = str[4..str.len()-1].split(",").collect();
        let lhs:i64 = parts[0].parse().unwrap();
        let rhs:i64 = parts[1].parse().unwrap();
        sum += lhs * rhs;
    });

    print!("Sum of valid mul expressions: {sum}");
}
