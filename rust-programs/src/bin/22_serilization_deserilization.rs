use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    // Create a User instance
    let u = User {
        name: "John".to_string(),
        age: 20,
    };

    // Serialize the struct into a JSON string
    let json_string = serde_json::to_string(&u).unwrap();
    println!("Serialized JSON: {}", json_string);

    // Deserialize the JSON string back into a User struct
    let deserialized_user: User = serde_json::from_str(&json_string).unwrap();
    println!("Deserialized User: {:?}", deserialized_user);
}
        