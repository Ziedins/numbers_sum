fn main() {
    const GOAL:usize = 1000000000000;

    println!("This program sums digits of all natural numbers starting from 0 to {GOAL}!");

    let mut sum: usize = 0;
    let vec = vec![0; GOAL];

    vec.iter().enumerate().for_each(|(mut index, _)|{
        while index > 0 {
            sum += index % 10;
            index = index / 10;
        }
    });

    println!("Digit sum from 0 to {GOAL} is {}", sum);
}
