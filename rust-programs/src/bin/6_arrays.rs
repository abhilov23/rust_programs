#[allow(unused)]

fn main(){
    //Array: fixed length, known at compile type
    //Slice: length not known at compile type
    

    //array
    let arr:[u32;4] = [1,2,3,4]; //immutable array
    println!("arr[2]:{}", arr[2]);

    let mut arr1:[u32;3] = [1,2,3]; //mutable array
    arr1[2] = 4;
    
    let arr2 : [u32;4] = [0;4]; //all four elements are 0
    println!("{:?}", arr2);
    
    //Slice
    let nums: [i32;10] = [-1,3,-5,6,5,3,-5,3,-7,-8];
    //taking the first 3 elements pof nums in the reference
    let s = &nums[0..3]; 
    println!("{:?}", s); //printing the first 3 elements

    //taking the first middle 4 elements
    let s2 = &nums[4..8]; 
    println!("{:?}", s2); //printing the first middle 4 elements

    //taking all the elements
    let s3 = &nums[..];
    println!("{:?}", s3); //printing all the elements
}