fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("The largest number is {}", largest(&number_list));

    let number_list = vec![35, 50, 75, 25, 100, 120, 95];

    println!("The largest number is {}", largest(&number_list));

    let char_list = vec!['e', 'a', 'f', 'i', 'm'];

    println!("The largest number is {}", largest(&char_list));
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}