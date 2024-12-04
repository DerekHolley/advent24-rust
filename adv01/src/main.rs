fn main() {
    let start = std::time::Instant::now();
    let (left,right) = match adv01::get_lists("input.txt") {
        Ok((l,r)) => (l,r),
        Err(e) => panic!("Analysis failed: {:?}", e)
    };
    
    let sum_of_difs = adv01::calculate_sum_of_differences(&left, &right);
    let similatiry = adv01::calculate_similarity(&left, &right);

    print!("Sum of differences:\n{}\nSimilarity:\n{}\n", sum_of_difs, similatiry);
    print!("Analysis took: {:?}\n", start.elapsed());
}

