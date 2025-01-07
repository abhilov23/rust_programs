#[allow(unused)]

fn main(){
  // +, -, *, /
  let a: i32 = 1;
  let b: i32 = 2;

  let c: i32 = a + b;
  let c = a - b;

  let c = a * b;
  //division rounds down to the nearest
  let c= a/b; //so the output will be 0
 
  // % (remainder != mod (aka modulo) operator) means % operator works as remainder operator
      // modulo operator
    // -1 % 3 = 2
    // remainder
    // -1 % 3 = -1
    let a = -1;
    let b = 3;
    let rem = a % b;
    println!("{a} % {b} = {rem}");
    // In Rust, the % operator (modulus operator) behaves differently compared to many other languages because it returns the remainder with a sign that matches the numerator. This difference arises because Rust follows mathematical modulo rules rather than truncated division rules.

  // Literals
   let c = 2i32 - 3; //2 is of i32
    let c = 3u32 * 2; //3 is of u32
    let c = 1.23e3; // it means 1.23 * 10^3 = 1230
    // 1.23 x 1000000
    println!("1.23e6 = {c}");
    // Improve readability using under-score _
    let c = 1_000_000_000u32;

        // Type casting
        let a: u32 = 1;
        let b = a as f32; //converting a into f32
        println!("b = {b}");
         
           // Comparisions
    let a = 1;
    let b = 2;
    let c = a == b;
    let c = a != b;
    let c = a <= b;
    let c = a < b;
    let c = a >= b;
    let c = a > b;

  // Boolean
  let c = true && false;
  let c = true || false;
  let c = !true;
 
      // Bitwise operators: performing operations on binary base
    // a is 101 in binary
    let a: u8 = 5;
    // b is 011
    let b: u8 = 3;
    println!("a & b = {:03b}", a & b);
    println!("a | b = {:03b}", a | b);
    println!("a ^ b = {:03b}", a ^ b);
    println!("!a = {:03b}", !a);
    println!("1 << 3 = {}", 1u32 << 3);
    // 10 >> 2 = 1010 >> 2 = 10
    println!("10 >> 2 = {}", 10u32 >> 2);
}