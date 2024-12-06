fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();
    println!("First element: {:?}", iter.next());
    println!("Second element: {:?}", iter.next());

    // Correctly handles the end of the iterator.
    for i in 0.. {
        match iter.next() {
            Some(x) => println!("Element {}: {:?}", i + 1, x),
            None => break,
        }
    }
}