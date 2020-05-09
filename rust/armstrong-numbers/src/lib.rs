pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    let mut n = num;
    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }

    let len = digits.len() as u32;
    digits.iter().map(|val| val.pow(len)).sum::<u32>() == num
}

pub fn is_armstrong_number2(num: u32) -> bool {
    let as_string = num.to_string();
    let len = as_string.chars().count() as u32;

    as_string
        .chars()
        .map(|ch| ch.to_digit(10).unwrap().pow(len))
        .sum::<u32>()
        == num
}
