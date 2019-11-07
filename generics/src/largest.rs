fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest(&number_list));

    let number_list = vec![35, 50, 75, 25, 100, 120, 95];

    println!("The largest number is {}", largest(&number_list));
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}