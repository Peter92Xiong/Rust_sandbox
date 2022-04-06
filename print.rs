pub fn run(){
    //print to console
    println!("Hello my name is Peter Xiong from the print.rs file");

    //basic formatting
    println!("{}",1);
    println!("{} is from {}", "Peter", "North Carolina");

    // positional arguments
    
    println!("{0} is {1} and {0} is {2}","Peter", "Awesome", "Cool");

    
    //named arguments

    println!("{name} likes to play {activity}", name = "Peter", activity = "football");

    //placeholder traits

    println!("Binary: {:b} Hex: {:x} Octal: {:o}",10,10,10);

    //placeholder for debug trait
    //multi values
    println!("{:?}",(12,true,"hello"));

    //basic math

    println!("10+10={}", 10+10);
}