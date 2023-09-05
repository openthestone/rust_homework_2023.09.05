pub fn compare_string(x: &str, y: &str) -> bool {
    for i in 0..x.len() {
        if i == y.len() || x.chars().nth(i) > y.chars().nth(i) {
            return true;
        } else if x.chars().nth(i) < y.chars().nth(i) {
            return false;
        }
    }
    false
}