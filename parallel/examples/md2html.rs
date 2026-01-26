use pulldown_cmark::{html, Options, Parser};
use rayon::prelude::*;

fn render_markdown(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(md, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/*
git:(main) ✗ cargo run -p parallel --example md2html
   Compiling parallel v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/examples/md2html`
Doc 1 -> <h1>Title</h1>
<ul>
<li>item 1</li>
<li>item 2</li>
</ul>

Doc 2 -> <h2>Second</h2>
<p>Some <em>italic</em> text.</p>

Doc 3 -> <h3>Third</h3>
<table><thead><tr><th>a</th><th>b</th></tr></thead><tbody>
<tr><td>1</td><td>2</td></tr>
</tbody></table>
*/

fn main() {
    let docs = vec![
        "# Title\n\n- item 1\n- item 2",
        "## Second\n\nSome *italic* text.",
        "### Third\n\n| a | b |\n|---|---|\n| 1 | 2 |",
    ];

    let htmls: Vec<String> = docs.par_iter().map(|md| render_markdown(md)).collect();

    for (i, html) in htmls.iter().enumerate() {
        println!("Doc {} -> {}", i + 1, html);
    }
}