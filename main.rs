//use rand::prelude::*;

//mod print;
//mod vars;
//mod types;
//mod strings;
// mod tuples;
//mod arrays;
//mod vectors;
//mod conditionals;
//mod loops;
//mod functions;
mod pointer_ref;

struct Employee {
    name: String,
}

trait Greeting {
    fn hello(&self) {
        println!("Hello");
    }
}

impl Greeting for Employee {
    fn hello(&self) {
        println!("Hello, I am {}!", &self.name);
    }
}

fn main() {
    //println!("Hello, world!");
    //print::run();
    //vars::run();
    //types::run();
    //strings::run();
    // tuples::run();
    //arrays::run();
    //vectors::run();
    //conditionals::run();
    //loops::run();
    //functions::run();
    pointer_ref::run();

    // let employee1 = Employee{
    //     name: String:: from("John"),

    // };
    // employee1.hello();
}
