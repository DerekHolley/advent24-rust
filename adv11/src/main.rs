use std::{collections::HashMap, vec};

fn main() {
    let input = vec![554735 as i64, 45401, 8434, 0, 188, 7487525, 77, 7];

    let now = std::time::Instant::now();

    let mut cache: HashMap<(i64, u8), i64> = HashMap::with_capacity(256_000);

    let part_1: i64 = input.iter().map(|n| recurse(*n, 25, &mut cache)).sum();
    let part_2: i64 = input.iter().map(|n| recurse(*n, 75, &mut cache)).sum();

    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", part_2);
    println!("Elapsed time: {:?}", now.elapsed());
}

fn recurse(num: i64, count: u8, cache: &mut HashMap<(i64, u8), i64>) -> i64 {
    if cache.contains_key(&(num, count)) {
        return *cache.get(&(num, count)).unwrap();
    }

    if count == 1 {
        if num == 0 {
            return 1;
        }
        let len = num.ilog10() + 1;
        if len % 2 == 0 {
            return 2;
        } else {
            return 1;
        }
    }

    let result: i64;
    if num == 0 {
        result = recurse(1, count - 1, cache);
    } else {
        let len = num.ilog10() + 1;
        if len % 2 == 0 {
            let exp = 10_i64.pow(len / 2);
            result = recurse(num / exp, count - 1, cache) + recurse(num % exp, count - 1, cache);
        } else {
            result = recurse(num * 2024, count - 1, cache);
        }
    }

    cache.insert((num, count), result);
    return result;
}
