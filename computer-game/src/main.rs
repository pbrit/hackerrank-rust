
struct Prime {
    curr: u32,
    history: Vec<u32>,
}

impl Prime {
    fn new() -> Prime {
        Prime {
            curr: 1,
            history: Vec::new(),
        }
    }
}

impl Iterator for Prime {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.curr + 1;

            if self.history.iter().any(|&x| next % x == 0) {
               self.curr = next;
            } else {
                self.history.push(next);

                return Some(next);
            }
        }
    }
}

// O(n)

fn main() {
    let count = 1_000_000;

    let prime = Prime::new();

    let nums: Vec<u32> = prime.take(count).collect();

    println!("Next {0} prime numbers: {1:?}", count, nums);
}
