// Take an unisgned integer and round it to the nearest penny
// 50.0005 --> 50.01
// 50.0004 --> 50.00
pub fn round_to_penny(num: f64) -> f64 {

    let mut num = num;

    (num * 100.0).round() / 100.0
}