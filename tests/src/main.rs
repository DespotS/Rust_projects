fn main() {
    let mut x = 6; //make it mutable so it can take other values as well
    println!("your number is {x}");
    x = 5;
    println!("your new number is {x}");

    //shadowing

    let y = 6;
    let y = y + 2;

    {
        let y = y * 2;
        println!("the value of y in the inner scope is {y}");
    }
    println!("the value of y in the outer scope is {y}");

    // tuples
    let z: (i32, f64, u8) = (100, 5.4, 1);

    let one_hundred = z.0;
    println!("first number is {one_hundred}");
    let second_number = z.1;
    println!("second number is {second_number}");
    let third_number = z.2;
    println!("third number is {third_number}");

    // arrays
    let a: [i32; 6] = [0, 5, 2, 3, 5, 1];
    let first = a[0];
    println!("first number in the array is {first}");

    // functions

    //simple function
    fn some_functions() {
        println!("just do something");
    }
    some_functions();

    // functions with parameters

    fn new_function(x: i32) {
        println!("the value of x is now {x}");
    }

    new_function(10);

    // functions with return values

    fn five() -> i32 {
        5
    }

    let x = five();
    println!("new value of x is {x}");

    // another function example

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let y = add_one(6);
    println!("the new value of y is {y}");

    // if statements

    //let condition = true;   not possible as the type of number was determine at runtime
    //let nummber = if condition { 5 } else { 6 };
    //println!("the final number from condition is {number}");

    // reverse range

    for number in (1..10).rev() {
        println!("{number}")
    }
    println!("reversed list is finished")
}
