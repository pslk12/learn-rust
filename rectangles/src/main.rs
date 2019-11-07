#![allow(dead_code)]

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}


enum Foo {
    Bar,
    Baz,
    Qux(u32)
}


impl Rectangle {
    fn area (&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // test_rect();
    // test_enums();
    // test_if_let();
    test_enum2();
}


fn test_rect() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 40, height: 40 };
    println!("can rec1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rec1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[allow(unused_variables)]
fn test_enums() {

    let some_number = Some(5);
    let some_string = Some("a string");

    let no_number: Option<i32> = None;

    let x = 5;
    let y = Some(5);

    let sum = x + match y { None => 0, Some(i) => i};

    println!("{}", sum);
}

fn test_if_let() {
    let some_u8 = Some(24u8);

    if let Some(i) = some_u8 {
        println!("Some({})", i);
    }
}

fn test_enum2() {
    // Create example variables
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo::Baz = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
