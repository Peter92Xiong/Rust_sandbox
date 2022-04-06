/*
Primitive Types in RUST--
Integers: u8,i8,u16,i16,u32,i32,u64,i64,u128,i128 (numbers of bits they take in memory)

    u- is unsigned (all positive).
floats: f32,f64
Boolean (bool)
characters (char)
Tuples -(they are list something like that)
Arrays -fixed length in RUST

    ...vectors are growable arrays...


*/

pub fn run(){

    //deafult its "i32"
    let x = 1;

    //by default its "f64"
    let y = 2.5;

    //add explicit type
    let z: i64 =123456789;

    //find max size

    println!("MAX i32: {}", std::i32::MAX);
    println!("MAX i64: {}", std::i64::MAX);





    //Boolean

    let is_active = true;
    //or
    // let is_active: bool = true;

    println!("{:?}", (x,y,z,is_active)); 
    //this prints out the value at that time; good for debugging 

    //get boolean from expression

    let is_greater: bool  = 10 >5;

    println!("is 10 greater than 5? {:?}", is_active);

    let a1 = 'a';
    let face = '\u{1F600}';
    //this is an emoj face

    println!("{}",face);

}