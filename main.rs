use std::io;

fn main() {
    println!("Hello, world!");

    // String define
    let str_x = String::from("x");
    
    let test_bool: bool = false;
    println!("TEST {}", test_bool);

    // Primative data types
    let x: i32 = -42;
    let y: i32 = 100;

    println!("Signed Integer: {}", x);
    println!("Undersigned Integer: {}", y);

    // Random tester
    let cool: i32 = -50;
    println!("Very cool signed number number: {}", cool); 
    test(x, y);

    // Generate how many numbers in the fibonachi sequence wanted to see
    println!("Enter a random Number to calculate the fibonachi sequence! Enter x for random generation..");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
            .expect("Unable to read user input!");

    // If user does not want to type a value do a random number
    println!("{}", user_input);
    if user_input ==  str_x{
        for int in 0..15{
            println!("Fibonachi iteration: ({}) = {}", int, fib(int));
        }
        return;
    }


    // Take user input and get fibonachi value
    let user_int: i32 = user_input
        .trim()
        .parse()
        .expect("Value is not a number!");

    let fib_val: i32 = fib(user_int);
    println!("Fibonachi Sequence of ({}) = {}", user_int, fib_val);
    return;

}


/// Calculate two signed integers - test function
///
/// # Arguments 
///
/// * `x`: The value comes in negative "signed"
/// * `y`: The value comes in postive "signed"
///
/// # Returns-
///
/// The sum of x and y values
///
fn test(x: i32, y: i32) -> i32 {
    let epic: i32 = y+x; 
    println!("Test test {}", epic);
    return epic;
}


/// Calculate the fibonachi sequence based on the number recieved
///
/// # Arguments-
///
/// * `n`: integer value to get the fibonachi seq return
///
/// # Returns-
///
/// The next iteration of the fibonachi return
///
fn fib(n: i32) -> i32{
    if n <= 0{
        return 0;
    } else if n == 1{
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}
