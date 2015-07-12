pub fn sum_of_squares(n: u32) -> u32 {
    let mut x = 0;
    for i in (1..n + 1) {
        x += i.pow(2)
    }
    x
}
pub fn square_of_sum(n: u32) -> u32 {
    let mut x = 0;
    for i in (1..n + 1) {
        x += i
    }
    x.pow(2)
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
