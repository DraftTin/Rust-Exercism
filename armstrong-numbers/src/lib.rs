pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .filter_map(|d| d.to_digit(10))
        .collect();
    let power = digits.len() as u32;
    let sum: u32 = digits.iter().map(|&n| n.pow(power)).sum();
    sum == num
}
