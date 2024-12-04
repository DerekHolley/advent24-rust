
pub fn is_line_safe(values: &Vec<i64>) -> bool {
    if values.len() < 2 {
        return true;
    }
    let ascending = values[0] < values[1];

    if values.windows(2).any(|w:&[i64]| -> bool {
        let distance = (w[0] - w[1]).abs();
        (w[0] < w[1]) != ascending || distance < 1 || distance > 3
    }) {
        return false;
    }

    return true;
}

pub fn is_line_safe_buffered(values: &Vec<i64>) -> bool {
    if is_line_safe(values) {
        return true;
    }

    for i in 0..values.len() {
        let mut c = values.clone();
        c.remove(i);
        if is_line_safe(&c){
            return true;
        }
    }

    return false;
}