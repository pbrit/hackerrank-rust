#!/usr/bin/env run-cargo-script --debug

#[derive(Debug)]
struct SquareMatrix {
    size: usize,
    data: Vec<Vec<i8>>,
}

#[derive(Debug)]
struct ParseMatrixError {}

impl std::str::FromStr for SquareMatrix {
    type Err = ParseMatrixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<i8>> = s.split('\n')
                      .map(|x| {
                            let mut row = Vec::new();
                            let iter = x.split(' ')
                                    .filter(|x| !x.is_empty() )
                                    .map(|x| x.trim().parse().unwrap() );

                            for x in iter {
                                row.push(x);
                            }

                            row  
                        }   
                      )
                      .collect();

        let size = data.len();

        Ok(SquareMatrix { data: data, size: size })
    }
}

pub fn main() {
    use std::io; 

    let mut line0 = String::new();
    let mut matrix_line = String::new();

    io::stdin().read_line(&mut line0).unwrap(); 
    let size: u32 = line0.trim().parse().ok().expect("panic2");

    for _ in 1..size+1 {
        io::stdin().read_line(&mut matrix_line).unwrap();
    }

    let matrix: SquareMatrix = matrix_line.trim().parse().ok().expect("panic3");

    println!("Matrix: {:?}", matrix);
}