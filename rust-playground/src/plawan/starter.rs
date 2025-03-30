// @plawanrath

pub fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

pub fn print() {
    println!("Your playground is all set!!");
}

pub fn run() {
    print();
    let nums = vec![1, 2, 3];
    println!("Sum = {}", sum(&nums));
}