#[derive(Clone, Copy)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn main() {
    let input = match std::fs::read_to_string("input.txt") {
        Ok(s) => s,
        Err(e) => panic!("Failed to open file: {}", e),
    };
    let now = std::time::Instant::now();
    
    let valid_calibrations_part_1: i64 = input.lines().map(|l| analyze_line(l, false)).sum::<i64>();
    let valid_calibrations_part_2: i64 = input.lines().map(|l| analyze_line(l, true)).sum::<i64>();

    println!("Part 1: {valid_calibrations_part_1}");
    println!("Part 2: {valid_calibrations_part_2}");

    println!("Analysis took: {:?}", now.elapsed());
}

fn analyze_line(line: &str, enable_concat:bool) -> i64 {
    let split_index = line.find(':').unwrap();
    let result: i64 = line[0..split_index].parse().unwrap();
    let parts: Vec<i64> = line[split_index + 2..]
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut operations: Vec<Operation> = vec![Operation::Add; parts.len() - 1];

    loop {
        let mut line_result = parts[0];
        for i in 1..parts.len() {
            let part = parts[i];
            let operation = operations[i - 1];
            line_result = match operation {
                Operation::Add => line_result + part,
                Operation::Multiply => line_result * part,
                Operation::Concatenate => concatenate(line_result, part),
            }
        }
        if line_result == result {
            return result;
        }
        if !increment_operations(&mut operations, enable_concat) {
            return 0;
        }
    }
}

fn increment_operations(operations: &mut Vec<Operation>, enable_concat:bool) -> bool {
    for i in 0..operations.len() {
        match operations[i] {
            Operation::Add => {
                operations[i] = Operation::Multiply;
                return true;
            },
            Operation::Multiply => {
                if enable_concat {
                    operations[i] = Operation::Concatenate;
                    return true;
                } else {
                    if i == operations.len() - 1 {
                        return false;
                    }
                    operations[i] = Operation::Add;
                }
            },
            Operation::Concatenate => {
                if i == operations.len() - 1 {
                    return false;
                }
                operations[i] = Operation::Add;
            },
        };
    }
    return true;
}

fn concatenate(a: i64, b: i64) -> i64 {
    let log = b.ilog10();
    (a * 10_i64.pow(log + 1)) + b
}
