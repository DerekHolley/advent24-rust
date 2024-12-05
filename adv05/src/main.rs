use std::fs;

fn main() {
    let contents = match fs::read_to_string("./input.txt") {
        Ok(contents) => contents,
        Err(err) => panic!("Failed to read file: {:?}", err)
    };
    let now = std::time::Instant::now();
    let content_ref:&str = contents.as_ref();
    let split = match content_ref.find("\n\n") {
        Some(i) => i,
        None => panic!("Failed to find double-line-feed to split input.")
    };

    let rule_str = &content_ref[0..split];
    let sequence_str = &content_ref[split+2..];

    let rules:Vec<(usize,usize)> = rule_str.lines().map(|r| -> (usize,usize) {
        let split = r.find('|').unwrap();
        (r[..split].parse().unwrap(), r[split+1..].parse().unwrap())
    }).collect();

    let sequences = sequence_str.lines().map(|line| -> Vec<usize> {
        line.split(',').map(|n| -> usize { n.parse().unwrap() }).collect()
    });

    let mut sum_of_valid_sequences:usize = 0;
    let mut sum_of_invalid_sequences:usize = 0;
    sequences.for_each(|mut sequence| {
        let mut is_invalid = false;
        for i in 0..sequence.len() {
            for j in i+1..sequence.len() {
                if rules.iter().any(|rule| {
                    sequence[i] == rule.1 && sequence[j] == rule.0
                }) {
                    is_invalid = true;
                    (sequence[i], sequence[j]) = (sequence[j], sequence[i]);
                }
            }
        }
        if is_invalid {
            sum_of_invalid_sequences += &sequence[sequence.len() / 2];
        } else {
            sum_of_valid_sequences += &sequence[sequence.len() / 2];

        }
    });
    print!("Analysis took: {:?}\n", now.elapsed());
    print!("Sum of middles of valid sequences: {:?}\n", sum_of_valid_sequences);
    print!("Sum of middles of corrected sequences: {:?}\n", sum_of_invalid_sequences);
}
