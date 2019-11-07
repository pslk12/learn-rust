extern crate rand;


fn main() {
    for element in 0..20 {
        println!("{}", fib(element) );
    }
}

fn fib(n:i32) -> i32 {
    if n == 0 { 
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n - 2)
    }
}

// fn change(s: &mut String) {
//     s.push_str(" world")
// }

//fn less_than_five(x:i32) -> bool {
//    x < 5
//}
//
//fn random() -> i32 {
//    rand::thread_rng().gen_range(1, 10)
//}
//
//fn countdown(x:i32) {
//    let mut number = x;
//
//    while number > 0 {
//        println!("{}", number);
//        number = number - 1;
//    }
//}
