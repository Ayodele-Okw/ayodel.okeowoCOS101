// the iter() function fetches values of all elemtns in an array.

fn main() {
    let arr:[i32;4]= [10,20,30,40];
    println!("array is {:?}",arr);
    println!("Array size is :{}",arr.len());

    for val in arr.iter(){
        println!("value is :{}",val);
    }
}