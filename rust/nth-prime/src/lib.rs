pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|x| is_prime(*x))
        .nth(n as usize)
        .expect("Cannot find anything")
}

fn is_prime(n: u32) -> bool {
    let limit: u32 = (n as f64).sqrt() as u32;

    (2..=limit).all(|a| n % a != 0)
}
