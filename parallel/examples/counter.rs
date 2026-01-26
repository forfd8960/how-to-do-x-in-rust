use rayon::prelude::*;

/*
git:(main) ✗ cargo run -p parallel --example counter
   Compiling either v1.15.0
   Compiling crossbeam-utils v0.8.21
   Compiling crossbeam-epoch v0.9.18
   Compiling crossbeam-deque v0.8.6
   Compiling rayon-core v1.13.0
   Compiling rayon v1.11.0
   Compiling parallel v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.13s
     Running `target/debug/examples/counter`
333333833333500000
*/

fn main() {
    let sum: i64 = (1..=1_000_000)
        .into_par_iter()
        .map(|x| x * x)
        .sum();
    println!("{}", sum);
}