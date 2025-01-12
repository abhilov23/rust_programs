#[allow(unused)]

use std::str;
use std::env;

use my::hello;


mod my{ 
 pub fn hello() //in this module, this is a public function so we can call it outside of the module
    {
        println!("Hello, from my module!");
    }
}

mod yours{ //this is another module, like not my module na 
    pub fn hello() 
       {
           println!("Hello, from yours module!");
       }
   }




mod foo {
    pub fn print() {
        println!("foo");
    }
}

mod my1 {
    pub fn print() {
        println!("my");
    }

    // Private - cannot be called by main
    fn f() {
        println!("private");
    }

    // Nest modules
    pub mod a {
        pub fn print() {
            println!("a");
        }

        // Public struct
        pub struct S {
            pub name: String,
            // Private field
            id: u32,
        }

        pub fn build(name: String) -> S {
            S { name, id: 1 }
        }
    }

    // Private module
    // Cannot be called outside of this module
    mod b {
        pub fn print() {
            println!("b");
        }
    }

    fn g() {
        b::print();
    }

    // Go one level up in the module tree
    use super::foo;

    fn call_foo_print() {
        foo::print();
    }
}

use my1::a::print as a_print;

fn main() {
    my1::print();
    my1::a::print();
    a_print();
    let s = my1::a::build("rust".to_string());
}