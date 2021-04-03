/// Returns the result of multiplying odd numbers in a queue of numbers.
///
/// # Arguments
///
/// * `queue` - a list of numbers
///
fn mult_elem(mut queue: Vec<u32>) -> u32 {
    let elem = match queue.pop() {
        Some(number) =>  number,
        None => 1,
    };

    if queue.len() > 0 {
        if (elem % 2) == 0 {
            return 1 * mult_elem(queue);
        } else {
            return elem * mult_elem(queue);
        }
    }
    return 1;
}

fn main() {
    let queue = vec![8, 3, 4, 3, 5];
    println!("The result is: {:?}", mult_elem(queue));
}
