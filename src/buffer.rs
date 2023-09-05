pub fn buffer<T: Copy + for<'a> std::ops::AddAssign<&'a T>> (v: &Vec<T>) -> T {
    let mut sum: T = v[0];
    for i in &v[1..] {
        sum += i;
    }
    sum
}