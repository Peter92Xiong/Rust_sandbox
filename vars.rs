pub fn run(){
    let name = "Peter";

    let mut age = 16;

    println!("My name is {} and I am {} years old.", name, age);
    age = 17; //this will only work if mut is added mut to "let mut age = 16;"

    println!("One year has passed, My name is {} and I am {} years old.", name, age);


    //define constant, when defining it always have to be define a type, i.e. interger 32 bit.
    //usually all const is all uppercase.

    const ID: i32 = 001;

    println!("ID: {}", ID);


    //assign multiple vars at once

    let (my_name, my_age) = ("Peter", 17);

    println!("assigning multiple vars at once: {} is {}",my_name,my_age);


}