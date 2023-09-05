mod buffer;
mod strcmp;

fn main() {

    // buffer
    let v1: Vec<i32> = vec![2, 5, 8, 11];
    println!("The sum of {:?} is {}.", v1, buffer::buffer(&v1)); // 26
    let v2: Vec<f32> = vec![2.2, 5.5, 8.8];
    println!("The sum of {:?} is {}.", v2, buffer::buffer(&v2)); // 16.5

    // strcpy
    let s1: &str = "abc";
    let s2: &str = "abd";
    println!("The comparing result of \"{}\" and \"{}\" is {}.", s1, s2, strcmp::compare_string(&s1, &s2)); // false
    println!("The comparing result of \"{}\" and \"{}\" is {}.", s1, s1, strcmp::compare_string(&s1, &s1)); // false
    println!("The comparing result of \"{}\" and \"{}\" is {}.", s2, s1, strcmp::compare_string(&s2, &s1)); // true
    let s3: &str = "ab";
    let s4: &str = "ac";
    println!("The comparing result of \"{}\" and \"{}\" is {}.", s1, s4, strcmp::compare_string(&s1, &s4)); // false
    println!("The comparing result of \"{}\" and \"{}\" is {}.", s1, s3, strcmp::compare_string(&s1, &s3)); // true
    println!("The comparing result of \"{}\" and \"{}\" is {}.", s3, s1, strcmp::compare_string(&s3, &s1)); // false

    // iterator
    let v3: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let iter = v3.iter().map(|&x| (x as u8 + 1) as char);
    let v4: Vec<char> = iter.collect();
    println!("The new vector is {:?}.", v4); // ['b', 'c', 'd', 'e', 'f']
}
