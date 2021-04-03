use std::io;

fn add_digits(num: u32) -> u32 {
    if num != 0 {
        let digit = num % 10;
        if digit >= 5 {
            return digit + add_digits(num / 10);
        } else {
            return add_digits(num / 10);
        }
    }

    return 0;
}

fn main() {
    let mut input_number = String::new();
    println!("Please type a number for calculating digits sum (works with digits above 5): ");
    io::stdin().read_line(&mut input_number).expect("could not read string");
    let digit_number: u32 = input_number.trim().parse().expect("Please digit an integer > 0");
    
    println!("SUM of {:?} is => {:?}", digit_number, add_digits(digit_number));
}
