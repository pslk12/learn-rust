const ATOZ :&str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn is_valid(s: &str) -> bool {
    if s.len() < 12 {
        println!("len < 12");
        return false;
    }

    if !has_digits(s) {
        println!("has_digits failed");
        return false;
    }

    if !has_alpha(s){
        println!("has_alpha failed");
        return false;
    }

    if !has_special(s){
        println!("has_special failed");
        return false;
    }

    if has_consecutive_letters(&(s.to_uppercase())){
        println!("has_consecutive_letters failed");
        return false;
    }

    return true;
}


fn has_digits(s: &str) -> bool {
    s.find(char::is_numeric) != None
}

fn has_alpha(s: &str) -> bool {
     s.find(char::is_uppercase) != None && 
     s.find(char::is_lowercase) != None
}

fn has_special(s: &str) -> bool {
    const SPECIALS :&str = "~!@#$%^&*()_?><:;'";

    s.chars().any(|c| SPECIALS.contains(c))
}

fn reverse(s: &str) -> String{
    s.chars().rev().collect::<String>()
}


fn has_consecutive_letters(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }

    for i in 0..s.len() - 3 {
        if ATOZ.contains(&s[i..i+3]) ||
           ATOZ.contains(&reverse(&s[i..i+3]))  {
            return true;
        }
    }

    false
}
