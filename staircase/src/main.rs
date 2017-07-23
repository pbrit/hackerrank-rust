use std::io;

fn main() {
    let mut staircase_len_str = String::new();
    io::stdin().read_line(&mut staircase_len_str).unwrap();

    let staircase_len: usize = staircase_len_str.trim().parse().unwrap();

    for i in 1..staircase_len+1 {
        let blank_len = staircase_len - i;

        println!("{0}{1}", " ".repeat(blank_len), "#".repeat(i));
    }
}
