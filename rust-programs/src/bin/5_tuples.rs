#![allow(unused)]



//compound data types
// - tuple
// - array

fn main(){
   //Tuple
   let t: (bool, u32, char) = (true, 45, 'a');
   //Destructure
   let (a, b, c) = t;
   //ignore with _
   let (_, b, _) = t; //we are ignoring the vaalues using _
   
   //Empty tuple - uint type
   let t = (); //empty tuple
   //Nested tuple
   let nested = ((1.23, 'a'),(true, 1u32, 'b'),());

   let f: (bool, u32, char) = (true, 45, 'a');
   let u = f.0;
   let g = f.1;
   let h = f.2;

   println!("{:?}", nested.0.0);
}