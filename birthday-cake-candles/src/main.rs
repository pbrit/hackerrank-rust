use std::io;

fn main() {
    let mut line0 = String::new();
    let mut line1 = String::new();

    io::stdin().read_line(&mut line0).unwrap();
    io::stdin().read_line(&mut line1).unwrap();

    let mut max = 1u32;
    let mut max_len = 0u32;

    for s in line1.split_whitespace() {
        let val: u32 = s.parse().unwrap();

        // Change max value and start counting over 
        if val == max {
            max_len += 1;
        } else if val > max {
            max = val;
            max_len = 1;
        }
    }

    println!("{}", max_len); 
}
