use std::io;

fn main() {

    println!("This program sums digits of all natural numbers starting from x to y!");


    let x: i64;

    loop {

        println!("Enter x (int)");
        let mut x_input = String::new();
        io::stdin()
            .read_line(&mut x_input)
            .expect("Failed to read input");

        x = match x_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Please enter positive number for side length!"); continue},
        };

        break;
    }


    let y: i64;

    loop {

        println!("Enter y (int)");
        let mut y_input = String::new();
        io::stdin()
            .read_line(&mut y_input)
            .expect("Failed to read input");

        y = match y_input.trim().parse() {
            Ok(num) => {
                if num <= x {
                    println!("y should be bigger than x");
                    continue;
                }

                num
            },
            Err(_) => {println!("Please enter a positive number!"); continue},
        };

        break;
    }

    let mut counter: i64 = x.clone();
    let mut sum: u32 = 0;

    while counter <= y {
        let number_string = counter.to_string();
        sum += number_string.chars().map(|char| char.to_digit(10).unwrap()).sum::<u32>();

        counter += 1;
    }

    println!("Digit sum from {x} to {y} is {}", sum);
}
