//vectors
//

pub fn run(){
/*------------type vec<type>----declare vec!*/
    let mut number: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", number);

    println!("single value: {}", number[0]);

    
    //change vaule
    number[2] = 20;
    number.push(5);
    number.push(6);
    number.pop(); //pop the last thing
    
    println!("{:?}", number);
    println!("Hello, my name is Peter Xiong. from vector file");

    println!("vector length: {}", number.len());

    //get slice of an array

    let slice: &[i32] = &number;//&number[1..3]; //":" set the type  the "[1..3]" means get the vaule at that index of the array

    println!("slicing: {:?}", slice);


    //loop through vector values
    for x in number.iter(){
        println!("Number in the for loop: {}", x);
    }

    //loop & mutate values //everything is times by 2.
    for x in number.iter_mut(){
        *x *=2;
    }
    println!("Numbers Vec for loop in mutate: {:?}",number );

    

}