#![allow(unused)]

fn main() {
    // Loop
    let mut i: u32 = 0;
    loop {
        println!("loop {}", i);
        i += 1;

        if i > 5 {
            break;
        }
    }

    // While loop
    let mut i: u32 = 0;
    while i <= 5 {
        println!("while loop {}", i);
        i += 1;
    }

    // For loop
    for i in 0..5 {
        println!("for loop {}", i);
    }

    // Loop array
    let xs = [1, 2, 3];

    // usize
    let n: usize = xs.len();
    for i in 0..n { //if you want to loop through the vector you can only do it like this for once
        // This will not compile
        // i is usize
        // let k = i + 1u32;
        println!("{}", xs[i]);
    }

    // Loop using iterator
    for x in xs.iter() { //but if you want to loop through the vector(hashmap, arrays) again use iter() method
        println!("for loop iter {}", x);
    }

    // Return value from loop: we need to place a comma when returning a value from a loop 
    let mut i: u32 = 0;
    let v = loop {
        i += 1;
        if i > 3 {
            break "i > 3";
        }
    };
    println!("return value from loop {}", v);

    // Labels
    let mut i = 0;
    'outer: for i in 0..3 { //defines the inner loop, if you want to break one loop or two, so we have 'inner and 'outer keywords
        'inner: for j in 0..3 {
            println!("{i}, {j}");
            if i == 1 && j == 1 {
                break 'inner;
            }
        }
    }
}