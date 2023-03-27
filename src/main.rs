fn main() {
    const GOAL:usize = 1000000000000;

    println!("This program sums digits of all natural numbers starting from 0 to {GOAL}!");

    let mut sum: usize = 0;
    let vec = vec![0; GOAL];

    vec.iter().enumerate().for_each(|(index, _)|{
        let mut number = index;
        while number > 0 {
            sum += number % 10;
            number = number / 10;
        }
    });

    println!("Digit sum from 0 to {GOAL} is {}", sum);
}
