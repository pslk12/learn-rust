
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }
}

fn main() {
//    let s = String::from("Hello World");
//    let hello = &s[0..5];
//    let world = &s[6..];

//    println!("h{}, w{}", hello, world);
//
//    println!("first_word({}) is {}", s, first_word(&s));

    let user1 = build_user(String::from("test"), String::from("test@test.com"));
    let user2 = User { name: String::from("tester"), email: String::from("tester@test.com"), ..user1};

    print_user(&user1);
    print_user(&user2);


    test_struct();
}

fn build_user(name: String, email: String) -> User {
    User {
        name, email, sign_in_count:1, active:true
    }
}

fn print_user(user: &User) {
    println!("name:{}, email:{}, signin:{}, active:{}", user.name, user.email, user.sign_in_count, user.active);
}

fn test_struct() {
    let rect = Rectangle { width: 30, height: 20 };
    println!("area of the rectangle({:?} is {}", rect, rect.area());
}



#[allow(dead_code)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
