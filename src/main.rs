use std::io;

fn main() {

    println!("This program sums digits of all natural numbers starting from 0 to y!");

    let y: i64;

    loop {

        println!("Enter y (int)");
        let mut y_input = String::new();
        io::stdin()
            .read_line(&mut y_input)
            .expect("Failed to read input");

        y = match y_input.trim().parse() {
            Ok(num) => {
                if num <= 0 {
                    println!("y should be bigger than 0");
                    continue;
                }

                num
            },
            Err(_) => {println!("Please enter a positive number!"); continue},
        };

        break;
    }

    let mut sum: i64 = 0;

    for counter in 0..y {
        let mut number = counter.clone();
        while number > 0 {
            sum += number % 10;
            number = number / 10;
        }
    }

    println!("Digit sum from 0 to {y} is {}", sum);
}
