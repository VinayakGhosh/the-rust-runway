struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    // Structs allow you to group related data together to form a custom data type.

    let mut user1 = User{
        email: String::from("abc@gmail.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count:1
    };
    let _name = user1.username;
    user1.username = String::from("newace2025");


    let user2 = build_user(
        String::from("rajkshamani@gmail.com"), 
        String::from("rajshamii")
    );

    // Reusing instance data
    let _user3 = User{
        email: String::from("madhav@gmail.com"),
        username: String::from("madhok"),
        ..user2                
    };



    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

}

fn build_user(email: String, username: String) -> User{
    User{
        email: email,
        username: username,
        sign_in_count: 2,
        active: true
    }
}
