pub fn run(){
    //print to console
    println!("Hello from print.rs file");

    //basic Formatting
    println!("{} is from {}", "Bob", "Mass");

    //positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Bob", "Mass", "Code");


    //Named Arguments
    println!("{} likes to play {}", name="John", activity="Football");

    //placeholder traits
    println!("binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //basic math
    println!("10 + 10 = {}", 10+10);
}