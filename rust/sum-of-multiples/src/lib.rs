pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|val| {
            factors
                .iter()
                .filter(|&fac| *fac != 0)
                .map(|factor| val % factor)
                .any(|r| r == 0)
        })
        .sum()
}
