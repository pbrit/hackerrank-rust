#!/usr/bin/env run-cargo-script --debug

use std::ops::Index;

const ITER_PRIMARY: u8 = 0;
const ITER_SECONARY: u8 = 1;

struct SquareMatrixIter<'a, T: 'a> {
    iter_type: u8, 
    curr: usize, 
    matrix: &'a SquareMatrix<T>,
}

#[derive(Debug)]
struct SquareMatrix<T> {
    size: usize,
    data: Vec<Vec<T>>,
}

#[derive(Debug)]
struct ParseMatrixError {}

impl <'a, T: Copy> Iterator for SquareMatrixIter<'a, T> {
    type Item = T; 

    fn next(&mut self) -> Option<T> {
        if self.curr == self.matrix.size {
            return None;
        }

        let old_curr =  self.curr;

        self.curr += 1;

        match self.iter_type {
            ITER_PRIMARY => return Some(*(self.matrix).data[old_curr].index(old_curr)),
            ITER_SECONARY => return Some(*(self.matrix).data[self.matrix.size - old_curr - 1].index(old_curr)),
            _ => panic!("Unsupported ITER_TYPE"),
        };
    }
}

impl <'a, T> SquareMatrix<T> {
    fn pdiag_iter(&self) -> SquareMatrixIter<T> {
        return SquareMatrixIter {
            matrix: self,
            curr: 0,   
            iter_type: ITER_PRIMARY,
        };
    }

    fn sdiag_iter(&self) -> SquareMatrixIter<T> {
        return SquareMatrixIter {
            matrix: self,
            curr: 0,
            iter_type: ITER_SECONARY,
        };
    }
}

impl <T> std::str::FromStr for SquareMatrix<T> where 
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug
{
    type Err = ParseMatrixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<Vec<T>> = s.split('\n')
                      .map(|x| {
                            let mut row = Vec::new();
                            let iter = x.split(' ')
                                    .map(|x| {
                                        x.parse().ok()
                                        .expect(format!("Panic on {0}:{1}", file!(), line!()).as_str())
                                    });

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
    let size: u32 = line0.trim().parse().ok()
                         .expect(format!("Panic on {0}:{1}", file!(), line!()).as_str());

    for _ in 1..size+1 {
        io::stdin().read_line(&mut matrix_line).unwrap();
    }

    let matrix: SquareMatrix<i32> = matrix_line.trim().parse().ok()
                                               .expect(format!("Panic on {0}:{1}", file!(), line!()).as_str());

    let pdiag_sum = matrix.pdiag_iter().fold(0, |acc, x| acc + x);
    let sdiag_sum = matrix.sdiag_iter().fold(0, |acc, x| acc + x);

    let result = (pdiag_sum - sdiag_sum).abs();

    println!("{}", result);
}