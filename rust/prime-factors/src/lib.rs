pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut res = Vec::new();

    // Handle even composite numbers
    while num % 2 == 0 {
        res.push(2);
        num /= 2;
    }

    // Handle odd composite numbers
    let sqrt = (num as f64).sqrt() as u64;
    for i in (3..=sqrt).step_by(2) {
        while num % i == 0 {
            res.push(i);
            num /= i;
        }
    }

    // Handle a prime number remainder
    if num > 2 {
        res.push(num);
    }

    res
}
