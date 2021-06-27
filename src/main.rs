use crate::functions::concatenate;

mod functions;

fn main() {
    let a = 1;
    let b = 2;
    let sum = functions::add(a, b);
    println!("The sum of {} and {} is {}", a, b, sum);

    /*  Because Intergers already have a known size at compile time the value
        is copied over to the stack because cloning() ints is very cheap
    */

    let mut str_1 = String::from("Hello,");
    let str_2 = String::from(" World!");
    concatenate(&mut str_1, &str_2);
    println!("{}", &str_1);
}
