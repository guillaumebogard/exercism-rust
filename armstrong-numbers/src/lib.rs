pub fn is_armstrong_number(num: u32) -> bool {
    let number_of_digits = num.to_string().len() as u32;
    let digits = num.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let sum = digits.iter().fold(0, |acc, x| acc + x.pow(number_of_digits) as usize);

    sum == num as usize
}
