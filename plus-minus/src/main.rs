
use std::io;

fn main() {
    let mut count_str = String::new();
    let mut numbers_str = String::new();

    io::stdin().read_line(&mut count_str).unwrap();
    io::stdin().read_line(&mut numbers_str).unwrap();

    let count: f32 = count_str.trim().parse().unwrap();
    let numbers: Vec<i32> = numbers_str.trim().split(' ')
                                       .map(|s| s.parse().unwrap() )
                                       .collect();

    let mut zero = 0f32;
    let mut positive = 0f32;
    let mut negative = 0f32; 

    for x in numbers {
        if x == 0 {
            zero += 1f32;
        } else if x > 0 {
            positive += 1f32;
        } else {
            negative += 1f32;
        }
    }

    print!("{0}\n{1}\n{2}", positive/count, negative/count, zero/count);
}
