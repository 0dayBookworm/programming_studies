use std::convert::TryInto;

fn sock_merchant(_n: u32, ar: Vec<u32>) -> u32 {
    let mut new_ar = ar.clone();
    new_ar.sort();
    new_ar.dedup();
    let mut pairs = 0;
    for i in 0..new_ar.len() {
        let matches = ar.iter().filter(|&n_color| *n_color == new_ar[i]).count();
        pairs += matches / 2;
    }
    return pairs.try_into().unwrap();
}

fn main() {
    let numbers = vec![10,20,20,10,10,30,50,10,20];
    println!("The result is: {:?}",sock_merchant(9, numbers));
}
