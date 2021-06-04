//Vectors - Variable list where elements are the same data types
use std::mem;

pub fn run (){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    //re-assign value
    numbers[2] = 10;

    //add to vector
    numbers.push(5);
    numbers.push(6);

    //pop off vector
    numbers.pop();

    println!("{:?}", numbers);

    //get single val
    println!("single value: {}", numbers[0]);

    //get Vector length
    println!("Vector length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("slice: {:?}", slice);

    //loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *=2;
    }

    println!("Numbers Vec: {:?}", numbers);
}