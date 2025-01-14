use std::io::{self, BufRead};

fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0; // LCM of 0 with any number is 0
    }
    return (a * b).abs() / gcd(a, b); // Use the relationship between GCD and LCM
}

fn gcd (mut a:i32,mut b:i32) ->i32 {
    while b!=0 {
        let temp = b;
        b = a%b;
        a = temp;
    }
    return a; 
}
fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Iterate over the test cases
    for _ in 0..t {
        if let Some(Ok(line)) = lines.next() {
            let values: Vec<u64> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let (l, r) = (values[0], values[1]);
            if (2*l<=r){
                println!("{} {}", l, 2*l);
            }
            else {
                println!("-1 -1");
            }

            
        }
    }
}
