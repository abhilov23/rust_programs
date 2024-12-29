#![allow(unused)]

fn main(){
    //variables in rust are immutable by default
    let x:i32 = -123; //this is a signed immutable integer 
    let mut y = 123; //this is a signed mutable integer 
    y+=1; //this is a mutable integer
    //in most cases rust itself identify the type so you do not have to declate the type.

    let z = -123;
    
    const A : u32 = 1;
    let z: bool = true;

    let v: Vec<_> = vec![1,2,3]; //defines a vector
}