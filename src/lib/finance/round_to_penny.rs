// Take an unisgned integer and round it to the nearest penny
// 50.0005 --> 50.01
// 50.0004 --> 50.00
pub fn round_to_penny(num: u32) -> u32 {

    let mut num = num;

    // If the number is greater than 5, round up (Ceiling)
    if num % 10 > 5 {
        num = num + 10 - (num % 10);
    } else {
        num = num - (num % 10);
    }

    num
}