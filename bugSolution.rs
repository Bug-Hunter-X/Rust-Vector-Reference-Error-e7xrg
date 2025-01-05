fn main() {
    let mut vec = Vec::with_capacity(3);
    vec.push(1);
    vec.push(2);
    let x = vec[0]; // Create a copy instead of reference
    vec.push(3);
    println!("x = {}", x);
}