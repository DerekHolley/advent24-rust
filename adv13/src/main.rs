#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    target_x: i64,
    target_y: i64,
}

impl Machine {
    fn calculate_cost(&self) -> Option<i64> {
        // Cramer's Rule isn't going to be as fast as Gaussian elimination,
        // but it's easy enough for this system

        let determinant = self.a_x * self.b_y - self.a_y * self.b_x;
        let a_presses = (self.target_x * self.b_y - self.target_y * self.b_x) / determinant;
        let b_presses = (self.target_y * self.a_x - self.target_x * self.a_y) / determinant;
        // Those answers may not be exact - integer rounding applies
        // so we'll check out numbers before we return them
        if a_presses * self.a_x + b_presses * self.b_x == self.target_x {
            return Some(a_presses * 3 + b_presses);
        } else {
            return None;
        }
    }

    fn calculate_boosted_cost(&self) -> Option<i64> {
        let actual_target_x = self.target_x + 10_000_000_000_000;
        let actual_target_y = self.target_y + 10_000_000_000_000;
        // Same idea as above - using the scaled targets instead
        let determinant = self.a_x * self.b_y - self.a_y * self.b_x;
        let a_presses = (actual_target_x * self.b_y - actual_target_y * self.b_x) / determinant;
        let b_presses = (actual_target_y * self.a_x - actual_target_x * self.a_y) / determinant;

        if a_presses * self.a_x + b_presses * self.b_x == actual_target_x {
            return Some(a_presses * 3 + b_presses);
        } else {
            return None;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let now = std::time::Instant::now();
    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|spec| {
            let parts: Vec<&str> = spec.split("\n").collect();
            if parts.len() != 3 {
                panic!("Failed to parse machine spec: {spec}");
            }
            let a_x: i64 = parts[0][12..14].parse().unwrap();
            let a_y: i64 = parts[0][18..20].parse().unwrap();
            let b_x: i64 = parts[1][12..14].parse().unwrap();
            let b_y: i64 = parts[1][18..20].parse().unwrap();

            let prize_line = parts[2];
            let target_x: i64 = prize_line
                [prize_line.find("X=").unwrap() + 2..prize_line.find(",").unwrap()]
                .parse()
                .unwrap();
            let target_y: i64 = prize_line[prize_line.find("Y=").unwrap() + 2..prize_line.len()]
                .parse()
                .unwrap();

            Machine {
                a_x,
                a_y,
                b_x,
                b_y,
                target_x,
                target_y,
            }
        })
        .collect();

    let total_cost: i64 = machines
        .iter()
        .map(|machine| match machine.calculate_cost() {
            Some(n) => n,
            None => 0,
        })
        .sum();

    let boosted_cost: i64 = machines
        .iter()
        .map(|machine| match machine.calculate_boosted_cost() {
            Some(n) => n,
            None => 0,
        })
        .sum();

    println!("Analysis took: {:?}", now.elapsed());
    println!("Part 1: {total_cost}");
    println!("Part 2: {boosted_cost}");
}
