mod utils;

fn main() {
    print_valid("thiS is a string");
    print_valid("smalPasswrd");
    print_valid("largePassword1");
    print_valid("g00dPassword!");
    print_valid("smalLpasswd");
    print_valid("smalLpasswxy1@");
    print_valid("smalLpasswvu1@");
    print_valid("smalLpasswvx1@");
}


#[allow(dead_code)]
fn print_valid(s: &str) {
    if utils::password_utils::is_valid(s){
    println!("{} is valid", s);
    } else {
        println!("{} is invalid", s);
    }
}

