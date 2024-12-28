#![allow(unused)]
#[derive(Debug)]


struct Lang{
    language: String,
    version: String 
}

fn main(){
   let lang = "rust";
   println!("hello {}", lang); //you can print the variables like this
   println!("hello {} {}", lang, lang); //you can place multiple variables like this
   println!("hello {lang}"); //also you can place it like this as well.
   
   let x = 2;
   println!("{0} * {0} = {1}", x, x * x); //using the positional arguments

   let lang = Lang { //complex datatypes
    language: "rust".to_string(),
    version: "1.8.3".to_string()
   };
   print!("{:?} \n",lang); //going to print compound datatypes in a line
   print!("{:#?}",lang);  //going to print compound datatypes in a format
}

