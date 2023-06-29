fn main() {
    for number in 1..=100 {
        match (number % 3, number % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{} (Это число не подходит не под одно из условий,проверяю следующие значения...)", number),
        }
    }
}

