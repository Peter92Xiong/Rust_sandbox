// loops - used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    //infinite loop if no if statment to stop it from running
    loop {
        count += 1;
        println!(" Number: {}", count);
        if count == 20 {
            break;
        }
    }

    //while loop (FIZZBUZZ) //if number is divisiable by 3 print fizz, if by 5 print buzz, if by both print fizzbuzz

    // while count <= 100 {
    //     if count % 15 == 0 {
    //         //this mean the number is diviaible by 15;
    //         println!("FIZZBUZZ");
    //     } else if count % 3 == 0 {
    //         println!("FIZZ");
    //     } else if count % 5 == 0 {
    //         println!("BUZZ");
    //     } else {
    //         println!("{}", count);
    //     }

    //     count += 1;
    // }




    
    //For Range

    for x in 0..100{ //the range from 0-100
        if x % 15 == 0 {
            //this mean the number is diviaible by 15;
            println!("FIZZBUZZ");
        } else if x % 3 == 0 {
            println!("FIZZ");
        } else if x % 5 == 0 {
            println!("BUZZ");
        } else {
            println!("{}", x);
        }

    }
}
