//funtions - used to store blocks of code for re-use

pub fn run(){
greeting("SUP", "Mr. Smith");

//Bind funtion values to variables

let get_sum = add(5,5);
println!("Sum: {}",get_sum);


//Closure
let n3: i32 = 10;
let add_nums = |n1: i32, n2: i32 | n1 + n2 + n3; //this one we can use outside variables like the "n3" to this variable
println!("C sum: {}", add_nums(3,3));

}

fn greeting(greet: &str, name: &str){

    println!("{} {}, nice to meet you!", greet, name);

}


//funtion to return u use ->
fn add(n1:i32, n2: i32) -> i32{

    n1+n2
}