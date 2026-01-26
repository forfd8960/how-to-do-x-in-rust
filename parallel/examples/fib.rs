use rayon::prelude::*;

fn fib(n: u32) -> u64 {
    if n <= 1 { return n as u64; }
    fib(n - 1) + fib(n - 2)
}

/*
git:(main) ✗ cargo run -p parallel --example fib        
   Compiling parallel v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/examples/fib`
fib(35) = 9227465
fib(36) = 14930352
fib(37) = 24157817
fib(38) = 39088169
fib(39) = 63245986
fib(40) = 102334155
*/

fn main() {
    let inputs = vec![35, 36, 37, 38, 39, 40];
    let results: Vec<u64> = inputs
        .par_iter()
        .map(|&n| fib(n))
        .collect();

    for (n, v) in inputs.iter().zip(results.iter()) {
        println!("fib({}) = {}", n, v);
    }
}