//arrays are fixed lengths
//

pub fn run(){

    let mut number: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", number);

    println!("single value: {}", number[0]);


    //change vaule
    number[2] = 20;

    println!("{:?}", number);
    println!("Hello, my name is Peter Xiong. from array file");

    println!("Array length: {}", number.len());

    //get slice of an array

    let slice: &[i32] = &number[1..3]; //":" set the type  the "[1..3]" means get the vaule at that index of the array

    println!("slicing: {:?}", slice);




    

}