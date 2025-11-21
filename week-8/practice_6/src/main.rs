fn main() {
    // Create Two vector
    let v = vec![1, 2, 3, 4, 5, 6];
    let x = vec![1, 5, 6, 9, 10, 11];

    // Use a for loop to add elements of the vector
    for index in 0..6 {
        let sum = v[index] + x[index];
        println!("{:?}", sum);
    }
}
