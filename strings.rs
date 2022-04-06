/*
there 2 types of string primitive string which is a fixed length and immutable
the other string is growable, heap-allocated data structure


*/

pub fn run(){
    let hi = "Hi";
    //this is immutable, a fixed length

    let mut hello = String::from("Hello ");
    //this is growable. 


    println!("Length: {}", hello.len());
    
    //push a char onto a string
    hello.push('w');

    println!("concat string with a char: {}",hello);

    //push a string onto a string

    hello.push_str("orld!");

    println!("concat a string with a string: {}", hello);

    //capacity in bytes:

    println!("Capacity: {}",hello.capacity());


    //loop

    for word in hello.split_whitespace(){
        println!("for loop: {}", word);
    }


}