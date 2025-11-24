use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("coud not remove file");
    println!("file is removed");
}