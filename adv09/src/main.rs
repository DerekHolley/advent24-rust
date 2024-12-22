
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let sequence: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let part_1_checksum = calculate_part_1_checksum(&sequence);

    let part_2_checksum = calculcate_part_2_checksum(&sequence);

    println!("Part 1 Checksum: {part_1_checksum}");
    println!("Part 2 Checksum: {part_2_checksum}");
}

fn calculate_part_1_checksum(sequence: &Vec<u32>) -> u64 {
    let required_space: u32 = sequence.iter().sum();
    let mut disk: Vec<i32> = std::vec::Vec::with_capacity(required_space as usize);

    let mut blanks = false;
    for i in 0..sequence.len() {
        let n = sequence[i];
        for _ in 0..n {
            disk.push(match blanks {
                true => -1, // -1 means empty space
                false => i as i32 / 2,
            })
        }
        blanks = !blanks;
    }

    //begin the swaps
    let mut head: usize = 0;
    let mut tail: usize = disk.len() - 1;
    loop {
        // Loop past all the non-'-1' entries
        while disk[head] != -1 && head < tail {
            head += 1;
        }
        // Loop past all the '-1' entries
        while disk[tail] == -1 && head < tail {
            tail -= 1;
        }
        if head >= tail {
            break;
        };
        let tmp = disk[head];
        disk[head] = disk[tail];
        disk[tail] = tmp;
    }

    let mut checksum: u64 = 0; //size this at 64 to prevent potential overflow
    for i in 0..disk.len() {
        let value = disk[i];
        if value == -1 {
            break;
        };
        checksum += i as u64 * value as u64;
    }

    checksum
}

fn calculcate_part_2_checksum(sequence: &Vec<u32>) -> u64 {
    let mut is_next_empty = false;
    let required_space: u32 = sequence.iter().sum();
    let mut disk: Vec<Option<u16>> = std::vec::Vec::with_capacity(required_space as usize);

    sequence.iter().enumerate().for_each(|(index, length)| {
        let val = match is_next_empty {
            true => None,
            false => Some(index as u16 / 2),
        };
        is_next_empty = !is_next_empty;
        for _ in 0..*length {
            disk.push(val);
        }
    });

    let mut tail = disk.len() - 1;
    loop {
        while tail > 0 && disk[tail].is_none() {
            tail -= 1;
        }
        if tail <= 0 {
            break;
        }

        let mut tail_cursor = tail;
        while tail_cursor > 0 && disk[tail_cursor].is_some() && disk[tail].unwrap() == disk[tail_cursor].unwrap() {
            tail_cursor -= 1;
        }
        if tail_cursor <= 0 {
            break;
        }
        tail_cursor += 1;

        let tail_len = tail - tail_cursor + 1;

        for head in 0..tail {
            if disk[head].is_some() {
                continue;
            }
            let mut head_cursor = head;
            while head_cursor < tail && disk[head_cursor].is_none() {
                head_cursor += 1;
            }
            head_cursor -= 1;
            let head_len = head_cursor - head + 1;

            if tail_len > head_len {
                continue;
            }

            for i in 0..tail_len {
                let tmp = disk[head + i];
                disk[head + i] = disk[tail_cursor + i];
                disk[tail_cursor + i] = tmp;
            }
            break;
        }
        tail = tail_cursor - 1;
    }

    let checksum: u64 = disk
        .iter()
        .enumerate()
        .map(|(index, block)| match block {
            Some(value) => index as u64 * *value as u64,
            None => 0,
        })
        .sum();

    checksum
}
