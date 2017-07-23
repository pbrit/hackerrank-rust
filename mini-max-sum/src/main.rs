use std::io;

fn main() {
    let mut line0 = String::new();

    io::stdin().read_line(&mut line0).unwrap();

    let mut array: Vec<u64> = line0.split_whitespace()
                                   .map(|x| x.parse().unwrap() )
                                   .collect();

    array.sort();

    let fold_func = |acc, x| acc + x; 

    let max_sum = array.iter().skip(1).fold(0, &fold_func);
    let min_sum = array.iter().take(array.len()-1).fold(0, &fold_func);

    println!("{0} {1}", min_sum, max_sum);
}
