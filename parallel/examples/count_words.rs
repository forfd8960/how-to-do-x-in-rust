use rayon::prelude::*;
use std::fs;
use walkdir::WalkDir;

/*
git:(main) ✗ cargo run -p parallel --example count_words
   Compiling same-file v1.0.6
   Compiling walkdir v2.5.0
   Compiling parallel v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/examples/count_words`
./parallel/Cargo.toml: 25 words
./parallel/examples/counter.rs: 62 words
./parallel/examples/count_words.rs: 59 words
./parallel/src/main.rs: 6 words
*/

fn main() -> std::io::Result<()> {
    let files: Vec<_> = WalkDir::new("./parallel")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    let counts: Vec<(String, usize)> = files
        .par_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(path).ok()?;
            let words = content.split_whitespace().count();
            Some((path.display().to_string(), words))
        })
        .collect();

    for (path, words) in counts {
        println!("{}: {} words", path, words);
    }

    Ok(())
}