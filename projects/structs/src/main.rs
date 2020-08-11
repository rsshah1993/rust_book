struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// allows printing for debugging purposes
// of non-standard structs that don't have Display
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// area method as part of the Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // can create methods that take multiple params
    // including other Rectangle objects
    fn can_hold(&self, rect: &Rectangle) -> bool{
        if self.area() > rect.area() {
            true
        } else {
            false
        }
    }
    // can create functions in `impl` blocks
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user_email = String::from("someusername@email.com");
    let user_username = String::from("someuser");
    let user1 = build_user(user_email, user_username);

    // can update second user with following syntax:
    let user2 = User{
        email: String::from("seconduser@email.com"),
        username: String::from("seconduser"),
        // define all the rest of the params identically to `user1`
        ..user1
    };
    println!("{}", user2.email);

    // tuple structs (no named values):
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // calculating area example with function
    let width = 5;
    let height = 10;
    let rect1 = Rectangle {
        width,
        height,
    };

    println!(
        "The area of the rectangle is {}",
        area(&rect1)
    );
    // pretty-print rect struct
    println!(
        "Rect1: {:#?}",
        rect1
    );

    // calculate area with method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    let rect2 = Rectangle {
        width: 2,
        height: 2,
    };

    println!("Rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    // accessing functions from within `impl` blocks
    // we are namespacing the `square` function inside
    // the Rectangle struct. (Later: modules are also defined similarly.)
    let square_obj = Rectangle::square(3);
    println!("Square object: {:#?}", square_obj);
}

fn build_user(email: String, username: String) -> User {
    // can input variables directly because naming is the same
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}