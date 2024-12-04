use std::fs;
use std::fmt;

pub enum AdvError {
    ReadFileFailed,
    ParseFailed(String),
}

impl fmt::Debug for AdvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdvError::ReadFileFailed => write!(f, "could not read file"),
            AdvError::ParseFailed(s) => write!(f, "failed to parse line {}", s),
        }
    }
}

fn parse_line(s:&str) -> Result<(i64,i64), AdvError> {
    let parts:Vec<&str> = s.split("   ").collect();
    if parts.len() != 2 {
        return Err(AdvError::ParseFailed(s.to_string()))
    }
    let l = match parts[0].parse() {
        Ok(v) => v,
        Err(_) => return Err(AdvError::ParseFailed(s.to_string()))
    };
    let r = match parts[1].parse() {
        Ok(v) => v,
        Err(_) => return Err(AdvError::ParseFailed(s.to_string()))
    };
    Ok((l,r))
}

pub fn get_lists(file_name:&str) -> Result<(Vec<i64>,Vec<i64>), AdvError>{

    let input_str = match fs::read_to_string(file_name){
        Ok(s) => s,
        Err(_) => return Err(AdvError::ReadFileFailed)
    };

    let mut left:Vec<i64> = Vec::with_capacity(1000);
    let mut right:Vec<i64> = Vec::with_capacity(1000);
    
    let pairs:Result<Vec<(i64,i64)>,AdvError> = input_str.lines().filter(|s:&&str| -> bool {
        s.len() > 0
    }).map(parse_line).collect();
    
    match pairs {
        Ok(pairs_vec) => {
            pairs_vec.iter().for_each(|pair| -> () {
                left.push(pair.0);
                right.push(pair.1);
            });
        },
        Err(_) => return Err(AdvError::ParseFailed("failed to break iterator into vectors".to_string()))
    }

    left.sort();
    right.sort();

    Ok((left,right))
}

pub fn calculate_sum_of_differences(left:&Vec<i64>, right:&Vec<i64>) -> i64 {
    let mut sum = 0;
    for (i, l) in left.iter().enumerate() {
        sum += (l - right[i]).abs()
    }
    sum
}

pub fn calculate_similarity(left:&Vec<i64>, right:&Vec<i64>) -> i64 {
    let mut sum = 0;
    for (_, l) in left.iter().enumerate() {
        for (_, r) in right.iter().enumerate() {
            if l == r {
                sum += l
            }
        }
    }
    sum
}