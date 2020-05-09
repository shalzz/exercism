use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut map = HashSet::<u64>::new();

    for num in 2..=upper_bound {
        if !map.contains(&num) {
            res.push(num);
            let mut mul = 2 * num;
            while mul <= upper_bound {
                map.insert(mul);
                mul += num;
            }
        }
    }
    res
}
