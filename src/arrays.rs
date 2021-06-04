//Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run (){
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    //re-assign value
    numbers[2] = 10;

    println!("{:?}", numbers);

    //get single val
    println!("single value: {}", numbers[0]);

    //get array length
    println!("Arrray length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("slice: {:?}", slice);
}