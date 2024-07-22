struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point(i32, i32); // tuple struct
struct Empty; // unit struct

fn main() {
    let username = String::from("john");

    let point = Point(1, 2);

    let user1 = User {
        active: true,
        username,
        email: String::from("john@gmail.com"),
        sign_in_count: 12,
    };
    println!("User {} has email - {} ", user1.username, user1.email);
    
    println!("x = {}, y = {}", point.0, point.1);
}

/*

Other types:
- tuple struct : struct that have unnamed fields
- unit structs : structs which donot have any attributes


Storage - str attributes stored in heap, others in stack


*/
