fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());

    // Trying to access elements that don't exist.
    for i in 0..5 {
        match iter.next() {
            Some(x) => println!("Element {}: {:?}", i + 1, x),
            None => println!("Element {}: None", i + 1),
        }
    }
}