struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main(){
    let user1: User = User{
        username: String::from("Abhilov"),
        email: String::from("abhilovgupta01@gmail.com"),
        sign_in_count: 56,
        active: true,
    };
    println!("{}", user1.username);

    let user2: User = User{
    username: String::from("John Doe"),
    email: String::from("johndoe@gmail.com"),
    sign_in_count: 100,
    active: false,
    };
}