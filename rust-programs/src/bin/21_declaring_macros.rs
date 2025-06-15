macro_rules! my_macro {
    () => {
        println!("Hello, world!");
    };
}

fn main(){
    my_macro!();
}