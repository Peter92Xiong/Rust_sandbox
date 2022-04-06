//conditionals- used to check the condition of something and act on the result
//              -- its like an else if
pub fn run(){

let age: u8 = 21;
let check_id:bool = false;
//let knows_person_of_age = true;

//if else statement
if age >= 21 && check_id{
    println!("Bartender: What would you like to drink");
}else if age < 21 && check_id{
    println!("Sorry you'll have to leave");
}else{
    
    println!("Bartender: I'll need to see your ID")
    
    
}


//shorthanf if

let is_of_age = if age >= 21 {true} else {false};

println!("Is of age: {}", is_of_age);


}