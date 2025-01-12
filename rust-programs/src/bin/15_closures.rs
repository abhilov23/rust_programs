fn main() {
    let x = 5;

    // A closure that uses `x` from the surrounding scope
    let add_to_x = |num| {num + x};
    // |num| is the input
    //num + x is the return value which will be stored in a variable.

    let add_two_values = |a,b| a+b;
    //takes 2 values and return a + b in add_two_values variable.

    println!("{}", add_to_x(10)); // Output is 15
    println!("the output will be: {}", add_two_values(5,15));
}